use ::libc;

pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::intptr_t;
pub use crate::stdlib::uint8_t;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::curl_h::CURL;
pub use crate::multi_h::CURLM;
pub use crate::qcommon_h::netadr_t;
pub use crate::qcommon_h::netadrtype_t;
pub use crate::qcommon_h::netchan_t;
pub use crate::qcommon_h::netsrc_t;
pub use crate::qcommon_h::vm_t;
pub use crate::qcommon_h::NA_BAD;
pub use crate::qcommon_h::NA_BOT;
pub use crate::qcommon_h::NA_BROADCAST;
pub use crate::qcommon_h::NA_IP;
pub use crate::qcommon_h::NA_IP6;
pub use crate::qcommon_h::NA_LOOPBACK;
pub use crate::qcommon_h::NA_MULTICAST6;
pub use crate::qcommon_h::NA_UNSPEC;
pub use crate::qcommon_h::NS_CLIENT;
pub use crate::qcommon_h::NS_SERVER;
pub use crate::src::qcommon::cmd::Cbuf_ExecuteText;
pub use crate::src::qcommon::cmd::Cmd_Argv;
pub use crate::src::qcommon::common::Com_DPrintf;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::common::Hunk_AllocateTempMemory;
pub use crate::src::qcommon::common::Hunk_FreeTempMemory;
pub use crate::src::qcommon::cvar::Cvar_Set;
pub use crate::src::qcommon::cvar::Cvar_VariableString;
pub use crate::src::qcommon::files::FS_FCloseFile;
pub use crate::src::qcommon::files::FS_FOpenFileRead;
pub use crate::src::qcommon::files::FS_Read;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::connstate_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::e_status;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fontInfo_t;
pub use crate::src::qcommon::q_shared::glyphInfo_t;
pub use crate::src::qcommon::q_shared::markFragment_t;
pub use crate::src::qcommon::q_shared::orientation_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::CA_ACTIVE;
pub use crate::src::qcommon::q_shared::CA_AUTHORIZING;
pub use crate::src::qcommon::q_shared::CA_CHALLENGING;
pub use crate::src::qcommon::q_shared::CA_CINEMATIC;
pub use crate::src::qcommon::q_shared::CA_CONNECTED;
pub use crate::src::qcommon::q_shared::CA_CONNECTING;
pub use crate::src::qcommon::q_shared::CA_DISCONNECTED;
pub use crate::src::qcommon::q_shared::CA_LOADING;
pub use crate::src::qcommon::q_shared::CA_PRIMED;
pub use crate::src::qcommon::q_shared::CA_UNINITIALIZED;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::qcommon::q_shared::FMV_EOF;
pub use crate::src::qcommon::q_shared::FMV_IDLE;
pub use crate::src::qcommon::q_shared::FMV_ID_BLT;
pub use crate::src::qcommon::q_shared::FMV_ID_IDLE;
pub use crate::src::qcommon::q_shared::FMV_ID_WAIT;
pub use crate::src::qcommon::q_shared::FMV_LOOPED;
pub use crate::src::qcommon::q_shared::FMV_PLAY;
pub use crate::src::qcommon::vm::VM_Call;
pub use crate::tr_public_h::refexport_t;
pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::polyVert_t;
pub use crate::tr_types_h::refEntityType_t;
pub use crate::tr_types_h::refEntity_t;
pub use crate::tr_types_h::refdef_t;
pub use crate::tr_types_h::stereoFrame_t;
pub use crate::tr_types_h::textureCompression_t;
pub use crate::tr_types_h::GLDRV_ICD;
pub use crate::tr_types_h::GLDRV_STANDALONE;
pub use crate::tr_types_h::GLDRV_VOODOO;
pub use crate::tr_types_h::GLHW_3DFX_2D3D;
pub use crate::tr_types_h::GLHW_GENERIC;
pub use crate::tr_types_h::GLHW_PERMEDIA2;
pub use crate::tr_types_h::GLHW_RAGEPRO;
pub use crate::tr_types_h::GLHW_RIVA128;
pub use crate::tr_types_h::RT_BEAM;
pub use crate::tr_types_h::RT_LIGHTNING;
pub use crate::tr_types_h::RT_MAX_REF_ENTITY_TYPE;
pub use crate::tr_types_h::RT_MODEL;
pub use crate::tr_types_h::RT_POLY;
pub use crate::tr_types_h::RT_PORTALSURFACE;
pub use crate::tr_types_h::RT_RAIL_CORE;
pub use crate::tr_types_h::RT_RAIL_RINGS;
pub use crate::tr_types_h::RT_SPRITE;
pub use crate::tr_types_h::STEREO_CENTER;
pub use crate::tr_types_h::STEREO_LEFT;
pub use crate::tr_types_h::STEREO_RIGHT;
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;
pub use crate::ui_public_h::UIMENU_BAD_CD_KEY;
pub use crate::ui_public_h::UIMENU_INGAME;
pub use crate::ui_public_h::UIMENU_MAIN;
pub use crate::ui_public_h::UIMENU_NEED_CD;
pub use crate::ui_public_h::UIMENU_NONE;
pub use crate::ui_public_h::UIMENU_POSTGAME;
pub use crate::ui_public_h::UIMENU_TEAM;
pub use crate::ui_public_h::UI_CONSOLE_COMMAND;
pub use crate::ui_public_h::UI_DRAW_CONNECT_SCREEN;
pub use crate::ui_public_h::UI_GETAPIVERSION;
pub use crate::ui_public_h::UI_HASUNIQUECDKEY;
pub use crate::ui_public_h::UI_INIT;
pub use crate::ui_public_h::UI_IS_FULLSCREEN;
pub use crate::ui_public_h::UI_KEY_EVENT;
pub use crate::ui_public_h::UI_MOUSE_EVENT;
pub use crate::ui_public_h::UI_REFRESH;
pub use crate::ui_public_h::UI_SET_ACTIVE_MENU;
pub use crate::ui_public_h::UI_SHUTDOWN;
pub use crate::vm_local_h::vm_s;

pub use crate::client_h::clientConnection_t;
pub use crate::client_h::clientStatic_t;
pub use crate::client_h::serverInfo_t;
pub use crate::src::client::cl_console::Con_Close;
pub use crate::src::client::cl_main::cl_inGameVideo;
pub use crate::src::client::cl_main::clc;
pub use crate::src::client::cl_main::cls;
pub use crate::src::client::cl_main::re;
pub use crate::src::client::cl_scrn::SCR_AdjustFrom640;
pub use crate::src::client::cl_ui::uivm;

extern "C" {
    #[no_mangle]
    pub static mut s_soundtime: i32;
    #[no_mangle]
    pub fn CL_ScaledMilliseconds() -> i32;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cin_cache {
    pub fileName: [i8; 4096],
    pub CIN_WIDTH: i32,
    pub CIN_HEIGHT: i32,
    pub xpos: i32,
    pub ypos: i32,
    pub width: i32,
    pub height: i32,
    pub looping: crate::src::qcommon::q_shared::qboolean,
    pub holdAtEnd: crate::src::qcommon::q_shared::qboolean,
    pub dirty: crate::src::qcommon::q_shared::qboolean,
    pub alterGameState: crate::src::qcommon::q_shared::qboolean,
    pub silent: crate::src::qcommon::q_shared::qboolean,
    pub shader: crate::src::qcommon::q_shared::qboolean,
    pub iFile: crate::src::qcommon::q_shared::fileHandle_t,
    pub status: crate::src::qcommon::q_shared::e_status,
    pub startTime: i32,
    pub lastTime: i32,
    pub tfps: isize,
    pub RoQPlayed: isize,
    pub ROQSize: isize,
    pub RoQFrameSize: u32,
    pub onQuad: isize,
    pub numQuads: isize,
    pub samplesPerLine: isize,
    pub roq_id: u32,
    pub screenDelta: isize,
    pub VQ0: Option<
        unsafe extern "C" fn(
            _: *mut crate::src::qcommon::q_shared::byte,
            _: *mut libc::c_void,
        ) -> (),
    >,
    pub VQ1: Option<
        unsafe extern "C" fn(
            _: *mut crate::src::qcommon::q_shared::byte,
            _: *mut libc::c_void,
        ) -> (),
    >,
    pub VQNormal: Option<
        unsafe extern "C" fn(
            _: *mut crate::src::qcommon::q_shared::byte,
            _: *mut libc::c_void,
        ) -> (),
    >,
    pub VQBuffer: Option<
        unsafe extern "C" fn(
            _: *mut crate::src::qcommon::q_shared::byte,
            _: *mut libc::c_void,
        ) -> (),
    >,
    pub samplesPerPixel: isize,
    pub gray: *mut crate::src::qcommon::q_shared::byte,
    pub xsize: u32,
    pub ysize: u32,
    pub maxsize: u32,
    pub minsize: u32,
    pub half: crate::src::qcommon::q_shared::qboolean,
    pub smootheddouble: crate::src::qcommon::q_shared::qboolean,
    pub inMemory: crate::src::qcommon::q_shared::qboolean,
    pub normalBuffer0: isize,
    pub roq_flags: isize,
    pub roqF0: isize,
    pub roqF1: isize,
    pub t: [isize; 2],
    pub roqFPS: isize,
    pub playonwalls: i32,
    pub buf: *mut crate::src::qcommon::q_shared::byte,
    pub drawX: isize,
    pub drawY: isize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cinematics_t {
    pub linbuf: [crate::src::qcommon::q_shared::byte; 2097152],
    pub file: [crate::src::qcommon::q_shared::byte; 65536],
    pub sqrTable: [i16; 256],
    pub mcomp: [i32; 256],
    pub qStatus: [[*mut crate::src::qcommon::q_shared::byte; 32768]; 2],
    pub oldXOff: isize,
    pub oldYOff: isize,
    pub oldysize: isize,
    pub oldxsize: isize,
    pub currentHandle: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_14 {
    pub i: *mut u32,
    pub s: *mut u16,
}
/* *****************************************************************************
*
* Class:		trFMV
*
* Description:	RoQ/RnR manipulation routines
*				not entirely complete for first run
*
******************************************************************************/

static mut ROQ_YY_tab: [isize; 256] = [0; 256];

static mut ROQ_UB_tab: [isize; 256] = [0; 256];

static mut ROQ_UG_tab: [isize; 256] = [0; 256];

static mut ROQ_VG_tab: [isize; 256] = [0; 256];

static mut ROQ_VR_tab: [isize; 256] = [0; 256];

static mut vq2: [u16; 16384] = [0; 16384];

static mut vq4: [u16; 65536] = [0; 65536];

static mut vq8: [u16; 262144] = [0; 262144];

static mut cin: cinematics_t = cinematics_t {
    linbuf: [0; 2097152],
    file: [0; 65536],
    sqrTable: [0; 256],
    mcomp: [0; 256],
    qStatus: [[0 as *mut crate::src::qcommon::q_shared::byte; 32768]; 2],
    oldXOff: 0,
    oldYOff: 0,
    oldysize: 0,
    oldxsize: 0,
    currentHandle: 0,
};

static mut cinTable: [cin_cache; 16] = [cin_cache {
    fileName: [0; 4096],
    CIN_WIDTH: 0,
    CIN_HEIGHT: 0,
    xpos: 0,
    ypos: 0,
    width: 0,
    height: 0,
    looping: crate::src::qcommon::q_shared::qfalse,
    holdAtEnd: crate::src::qcommon::q_shared::qfalse,
    dirty: crate::src::qcommon::q_shared::qfalse,
    alterGameState: crate::src::qcommon::q_shared::qfalse,
    silent: crate::src::qcommon::q_shared::qfalse,
    shader: crate::src::qcommon::q_shared::qfalse,
    iFile: 0,
    status: crate::src::qcommon::q_shared::FMV_IDLE,
    startTime: 0,
    lastTime: 0,
    tfps: 0,
    RoQPlayed: 0,
    ROQSize: 0,
    RoQFrameSize: 0,
    onQuad: 0,
    numQuads: 0,
    samplesPerLine: 0,
    roq_id: 0,
    screenDelta: 0,
    VQ0: None,
    VQ1: None,
    VQNormal: None,
    VQBuffer: None,
    samplesPerPixel: 0,
    gray: 0 as *mut crate::src::qcommon::q_shared::byte,
    xsize: 0,
    ysize: 0,
    maxsize: 0,
    minsize: 0,
    half: crate::src::qcommon::q_shared::qfalse,
    smootheddouble: crate::src::qcommon::q_shared::qfalse,
    inMemory: crate::src::qcommon::q_shared::qfalse,
    normalBuffer0: 0,
    roq_flags: 0,
    roqF0: 0,
    roqF1: 0,
    t: [0; 2],
    roqFPS: 0,
    playonwalls: 0,
    buf: 0 as *mut crate::src::qcommon::q_shared::byte,
    drawX: 0,
    drawY: 0,
}; 16];

static mut currentHandle: i32 = -(1);

static mut CL_handle: i32 = -(1);
// sample PAIRS
#[no_mangle]

pub unsafe extern "C" fn CIN_CloseAllVideos() {
    let mut i: i32 = 0;
    i = 0;
    while i < 16 {
        if cinTable[i as usize].fileName[0] as i32 != 0 {
            CIN_StopCinematic(i);
        }
        i += 1
    }
}

unsafe extern "C" fn CIN_HandleForVideo() -> i32 {
    let mut _i: i32 = 0;

    for i in 0..16 {
        if cinTable[i as usize].fileName[0] as i32 == 0 {
            return i;
        }
    }
    crate::src::qcommon::common::Com_Error(
        crate::src::qcommon::q_shared::ERR_DROP as i32,
        b"CIN_HandleForVideo: none free\x00" as *const u8 as *const i8,
    );
}
//-----------------------------------------------------------------------------
// RllSetupTable
//
// Allocates and initializes the square table.
//
// Parameters:	None
//
// Returns:		Nothing
//-----------------------------------------------------------------------------

unsafe extern "C" fn RllSetupTable() {
    let mut z: i32 = 0;
    z = 0;
    while z < 128 {
        cin.sqrTable[z as usize] = (z * z) as i16;
        cin.sqrTable[(z + 128) as usize] = -(cin.sqrTable[z as usize] as i32) as i16;
        z += 1
    }
}
//-----------------------------------------------------------------------------
// RllDecodeMonoToMono
//
// Decode mono source data into a mono buffer.
//
// Parameters:	from -> buffer holding encoded data
//				to ->	buffer to hold decoded data
//				size =	number of bytes of input (= # of shorts of output)
//				signedOutput = 0 for unsigned output, non-zero for signed output
//				flag = flags from asset header
//
// Returns:		Number of samples placed in output buffer
//-----------------------------------------------------------------------------
#[no_mangle]

pub unsafe extern "C" fn RllDecodeMonoToMono(
    mut from: *mut u8,
    mut to: *mut i16,
    mut size: u32,
    mut signedOutput: i8,
    mut flag: u16,
) -> isize {
    let mut z: u32 = 0;
    let mut prev: i32 = 0;
    if signedOutput != 0 {
        prev = flag as i32 - 0x8000
    } else {
        prev = flag as i32
    }
    z = 0;
    while z < size {
        let ref mut fresh0 = *to.offset(z as isize);
        *fresh0 = (prev + cin.sqrTable[*from.offset(z as isize) as usize] as i32) as i16;
        prev = *fresh0 as i32;
        z = z.wrapping_add(1)
    }
    return size as isize;
    //*sizeof(short));
}
//-----------------------------------------------------------------------------
// RllDecodeMonoToStereo
//
// Decode mono source data into a stereo buffer. Output is 4 times the number
// of bytes in the input.
//
// Parameters:	from -> buffer holding encoded data
//				to ->	buffer to hold decoded data
//				size =	number of bytes of input (= 1/4 # of bytes of output)
//				signedOutput = 0 for unsigned output, non-zero for signed output
//				flag = flags from asset header
//
// Returns:		Number of samples placed in output buffer
//-----------------------------------------------------------------------------
#[no_mangle]

pub unsafe extern "C" fn RllDecodeMonoToStereo(
    mut from: *mut u8,
    mut to: *mut i16,
    mut size: u32,
    mut signedOutput: i8,
    mut flag: u16,
) -> isize {
    let mut z: u32 = 0;
    let mut prev: i32 = 0;
    if signedOutput != 0 {
        prev = flag as i32 - 0x8000
    } else {
        prev = flag as i32
    }
    z = 0;
    while z < size {
        prev = (prev + cin.sqrTable[*from.offset(z as isize) as usize] as i32) as i16 as i32;
        let ref mut fresh1 = *to.offset(z.wrapping_mul(2u32).wrapping_add(1u32) as isize);
        *fresh1 = prev as i16;
        *to.offset(z.wrapping_mul(2u32).wrapping_add(0u32) as isize) = *fresh1;
        z = z.wrapping_add(1)
    }
    return size as isize;
    // * 2 * sizeof(short));
}
//-----------------------------------------------------------------------------
// RllDecodeStereoToStereo
//
// Decode stereo source data into a stereo buffer.
//
// Parameters:	from -> buffer holding encoded data
//				to ->	buffer to hold decoded data
//				size =	number of bytes of input (= 1/2 # of bytes of output)
//				signedOutput = 0 for unsigned output, non-zero for signed output
//				flag = flags from asset header
//
// Returns:		Number of samples placed in output buffer
//-----------------------------------------------------------------------------
#[no_mangle]

pub unsafe extern "C" fn RllDecodeStereoToStereo(
    mut from: *mut u8,
    mut to: *mut i16,
    mut size: u32,
    mut signedOutput: i8,
    mut flag: u16,
) -> isize {
    let mut z: u32 = 0;
    let mut zz: *mut u8 = from;
    let mut prevL: i32 = 0;
    let mut prevR: i32 = 0;
    if signedOutput != 0 {
        prevL = (flag as i32 & 0xff00) - 0x8000;
        prevR = ((flag as i32 & 0xff) << 8) - 0x8000
    } else {
        prevL = flag as i32 & 0xff00;
        prevR = (flag as i32 & 0xff) << 8
    }
    z = 0;
    while z < size {
        let fresh2 = zz;
        zz = zz.offset(1);
        prevL = (prevL + cin.sqrTable[*fresh2 as usize] as i32) as i16 as i32;
        let fresh3 = zz;
        zz = zz.offset(1);
        prevR = (prevR + cin.sqrTable[*fresh3 as usize] as i32) as i16 as i32;
        *to.offset(z.wrapping_add(0u32) as isize) = prevL as i16;
        *to.offset(z.wrapping_add(1u32) as isize) = prevR as i16;
        z = z.wrapping_add(2u32)
    }
    return (size >> 1) as isize;
    //*sizeof(short));
}
//-----------------------------------------------------------------------------
// RllDecodeStereoToMono
//
// Decode stereo source data into a mono buffer.
//
// Parameters:	from -> buffer holding encoded data
//				to ->	buffer to hold decoded data
//				size =	number of bytes of input (= # of bytes of output)
//				signedOutput = 0 for unsigned output, non-zero for signed output
//				flag = flags from asset header
//
// Returns:		Number of samples placed in output buffer
//-----------------------------------------------------------------------------
#[no_mangle]

pub unsafe extern "C" fn RllDecodeStereoToMono(
    mut from: *mut u8,
    mut to: *mut i16,
    mut size: u32,
    mut signedOutput: i8,
    mut flag: u16,
) -> isize {
    let mut z: u32 = 0;
    let mut prevL: i32 = 0;
    let mut prevR: i32 = 0;
    if signedOutput != 0 {
        prevL = (flag as i32 & 0xff00) - 0x8000;
        prevR = ((flag as i32 & 0xff) << 8) - 0x8000
    } else {
        prevL = flag as i32 & 0xff00;
        prevR = (flag as i32 & 0xff) << 8
    }
    z = 0;
    while z < size {
        prevL = prevL + cin.sqrTable[*from.offset(z.wrapping_mul(2u32) as isize) as usize] as i32;
        prevR = prevR
            + cin.sqrTable[*from.offset(z.wrapping_mul(2u32).wrapping_add(1u32) as isize) as usize]
                as i32;
        *to.offset(z as isize) = ((prevL + prevR) / 2) as i16;
        z = z.wrapping_add(1u32)
    }
    return size as isize;
}
/* *****************************************************************************
*
* Function:
*
* Description:
*
******************************************************************************/

unsafe extern "C" fn move8_32(
    mut src: *mut crate::src::qcommon::q_shared::byte,
    mut dst: *mut crate::src::qcommon::q_shared::byte,
    mut spl: i32,
) {
    let mut i: i32 = 0;
    i = 0;
    while i < 8 {
        crate::stdlib::memcpy(dst as *mut libc::c_void, src as *const libc::c_void, 32);
        src = src.offset(spl as isize);
        dst = dst.offset(spl as isize);
        i += 1
    }
}
/* *****************************************************************************
*
* Function:
*
* Description:
*
******************************************************************************/

unsafe extern "C" fn move4_32(
    mut src: *mut crate::src::qcommon::q_shared::byte,
    mut dst: *mut crate::src::qcommon::q_shared::byte,
    mut spl: i32,
) {
    let mut i: i32 = 0;
    i = 0;
    while i < 4 {
        crate::stdlib::memcpy(dst as *mut libc::c_void, src as *const libc::c_void, 16);
        src = src.offset(spl as isize);
        dst = dst.offset(spl as isize);
        i += 1
    }
}
/* *****************************************************************************
*
* Function:
*
* Description:
*
******************************************************************************/

unsafe extern "C" fn blit8_32(
    mut src: *mut crate::src::qcommon::q_shared::byte,
    mut dst: *mut crate::src::qcommon::q_shared::byte,
    mut spl: i32,
) {
    let mut i: i32 = 0;
    i = 0;
    while i < 8 {
        crate::stdlib::memcpy(dst as *mut libc::c_void, src as *const libc::c_void, 32);
        src = src.offset(32);
        dst = dst.offset(spl as isize);
        i += 1
    }
}
/* *****************************************************************************
*
* Function:
*
* Description:
*
******************************************************************************/

unsafe extern "C" fn blit4_32(
    mut src: *mut crate::src::qcommon::q_shared::byte,
    mut dst: *mut crate::src::qcommon::q_shared::byte,
    mut spl: i32,
) {
    let mut i: i32 = 0;
    i = 0;
    while i < 4 {
        crate::stdlib::memmove(dst as *mut libc::c_void, src as *const libc::c_void, 16);
        src = src.offset(16);
        dst = dst.offset(spl as isize);
        i += 1
    }
}
/* *****************************************************************************
*
* Function:
*
* Description:
*
******************************************************************************/

unsafe extern "C" fn blit2_32(
    mut src: *mut crate::src::qcommon::q_shared::byte,
    mut dst: *mut crate::src::qcommon::q_shared::byte,
    mut spl: i32,
) {
    crate::stdlib::memcpy(dst as *mut libc::c_void, src as *const libc::c_void, 8);
    crate::stdlib::memcpy(
        dst.offset(spl as isize) as *mut libc::c_void,
        src.offset(8) as *const libc::c_void,
        8,
    );
}
/* *****************************************************************************
*
* Function:
*
* Description:
*
******************************************************************************/

unsafe extern "C" fn blitVQQuad32fs(
    mut status: *mut *mut crate::src::qcommon::q_shared::byte,
    mut data: *mut u8,
) {
    let mut newd: u16 = 0;
    let mut celdata: u16 = 0;
    let mut code: u16 = 0;
    let mut index: u32 = 0;
    let mut i: u32 = 0;
    let mut spl: i32 = 0;
    newd = 0;
    celdata = 0;
    index = 0;
    spl = cinTable[currentHandle as usize].samplesPerLine as i32;
    loop {
        if newd == 0 {
            newd = 7;
            celdata = (*data.offset(0) as i32 + *data.offset(1) as i32 * 256) as u16;
            data = data.offset(2)
        } else {
            newd = newd.wrapping_sub(1)
        }
        code = (celdata as i32 & 0xc000) as u16;
        celdata = ((celdata as i32) << 2) as u16;
        match code as i32 {
            32768 => {
                // vq code
                blit8_32(
                    &mut *vq8.as_mut_ptr().offset((*data as i32 * 128) as isize) as *mut u16
                        as *mut crate::src::qcommon::q_shared::byte,
                    *status.offset(index as isize),
                    spl,
                ); // skip 8x8
                data = data.offset(1);
                index = index.wrapping_add(5u32)
            }
            49152 => {
                // drop
                index = index.wrapping_add(1);
                i = 0;
                while i < 4 {
                    if newd == 0 {
                        newd = 7;
                        celdata = (*data.offset(0) as i32 + *data.offset(1) as i32 * 256) as u16;
                        data = data.offset(2)
                    } else {
                        newd = newd.wrapping_sub(1)
                    }
                    code = (celdata as i32 & 0xc000) as u16;
                    celdata = ((celdata as i32) << 2) as u16;
                    match code as i32 {
                        32768 => {
                            // code in top two bits of code
                            // 4x4 vq code
                            blit4_32(
                                &mut *vq4.as_mut_ptr().offset((*data as i32 * 32) as isize)
                                    as *mut u16
                                    as *mut crate::src::qcommon::q_shared::byte,
                                *status.offset(index as isize),
                                spl,
                            );
                            data = data.offset(1)
                        }
                        49152 => {
                            // 2x2 vq code
                            blit2_32(
                                &mut *vq2.as_mut_ptr().offset((*data as i32 * 8) as isize)
                                    as *mut u16
                                    as *mut crate::src::qcommon::q_shared::byte,
                                *status.offset(index as isize),
                                spl,
                            );
                            data = data.offset(1);
                            blit2_32(
                                &mut *vq2.as_mut_ptr().offset((*data as i32 * 8) as isize)
                                    as *mut u16
                                    as *mut crate::src::qcommon::q_shared::byte,
                                (*status.offset(index as isize)).offset(8),
                                spl,
                            );
                            data = data.offset(1);
                            blit2_32(
                                &mut *vq2.as_mut_ptr().offset((*data as i32 * 8) as isize)
                                    as *mut u16
                                    as *mut crate::src::qcommon::q_shared::byte,
                                (*status.offset(index as isize)).offset((spl * 2) as isize),
                                spl,
                            );
                            data = data.offset(1);
                            blit2_32(
                                &mut *vq2.as_mut_ptr().offset((*data as i32 * 8) as isize)
                                    as *mut u16
                                    as *mut crate::src::qcommon::q_shared::byte,
                                (*status.offset(index as isize))
                                    .offset((spl * 2) as isize)
                                    .offset(8),
                                spl,
                            );
                            data = data.offset(1)
                        }
                        16384 => {
                            // motion compensation
                            move4_32(
                                (*status.offset(index as isize))
                                    .offset(cin.mcomp[*data as usize] as isize),
                                *status.offset(index as isize),
                                spl,
                            );
                            data = data.offset(1)
                        }
                        _ => {}
                    }
                    index = index.wrapping_add(1);
                    i = i.wrapping_add(1)
                }
            }
            16384 => {
                // motion compensation
                move8_32(
                    (*status.offset(index as isize)).offset(cin.mcomp[*data as usize] as isize),
                    *status.offset(index as isize),
                    spl,
                );
                data = data.offset(1);
                index = index.wrapping_add(5u32)
            }
            0 => index = index.wrapping_add(5u32),
            _ => {}
        }
        if (*status.offset(index as isize)).is_null() {
            break;
        }
    }
}
/* *****************************************************************************
*
* Function:
*
* Description:
*
******************************************************************************/

unsafe extern "C" fn ROQ_GenYUVTables() {
    let mut t_ub: f32 = 0.;
    let mut t_vr: f32 = 0.;
    let mut t_ug: f32 = 0.;
    let mut t_vg: f32 = 0.;
    let mut i: isize = 0;
    t_ub = 1.77200 / 2.0 * ((1i32) << 6) as f32 + 0.5;
    t_vr = 1.40200 / 2.0 * ((1i32) << 6) as f32 + 0.5;
    t_ug = 0.34414 / 2.0 * ((1i32) << 6) as f32 + 0.5;
    t_vg = 0.71414 / 2.0 * ((1i32) << 6) as f32 + 0.5;
    i = 0;
    while i < 256 {
        let mut x: f32 = (2 * i - 255) as f32;
        ROQ_UB_tab[i as usize] = (t_ub * x + ((1i32) << 5) as f32) as isize;
        ROQ_VR_tab[i as usize] = (t_vr * x + ((1i32) << 5) as f32) as isize;
        ROQ_UG_tab[i as usize] = (-t_ug * x) as isize;
        ROQ_VG_tab[i as usize] = (-t_vg * x + ((1i32) << 5) as f32) as isize;
        ROQ_YY_tab[i as usize] = i << 6 | i >> 2;
        i += 1
    }
}
/* *****************************************************************************
*
* Function:
*
* Description:
*
******************************************************************************/

unsafe extern "C" fn yuv_to_rgb(mut y: isize, mut u: isize, mut v: isize) -> u16 {
    let mut r: isize = 0;
    let mut g: isize = 0;
    let mut b: isize = 0;
    let mut YY: isize = ROQ_YY_tab[y as usize];
    r = YY + ROQ_VR_tab[v as usize] >> 9;
    g = YY + ROQ_UG_tab[u as usize] + ROQ_VG_tab[v as usize] >> 8;
    b = YY + ROQ_UB_tab[u as usize] >> 9;
    if r < 0 {
        r = 0
    }
    if g < 0 {
        g = 0
    }
    if b < 0 {
        b = 0
    }
    if r > 31 {
        r = 31
    }
    if g > 63 {
        g = 63
    }
    if b > 31 {
        b = 31
    }
    return ((r << 11) + (g << 5) + b) as u16;
}
/* *****************************************************************************
*
* Function:
*
* Description:
*
******************************************************************************/

unsafe extern "C" fn yuv_to_rgb24(mut y: isize, mut u: isize, mut v: isize) -> u32 {
    let mut r: isize = 0;
    let mut g: isize = 0;
    let mut b: isize = 0;
    let mut YY: isize = ROQ_YY_tab[y as usize];
    r = YY + ROQ_VR_tab[v as usize] >> 6;
    g = YY + ROQ_UG_tab[u as usize] + ROQ_VG_tab[v as usize] >> 6;
    b = YY + ROQ_UB_tab[u as usize] >> 6;
    if r < 0 {
        r = 0
    }
    if g < 0 {
        g = 0
    }
    if b < 0 {
        b = 0
    }
    if r > 255 {
        r = 255
    }
    if g > 255 {
        g = 255
    }
    if b > 255 {
        b = 255
    }
    return ((r | g << 8 | b << 16) as usize | (255) << 24) as u32;
}
/* *****************************************************************************
*
* Function:
*
* Description:
*
******************************************************************************/

unsafe extern "C" fn decodeCodeBook(
    mut input: *mut crate::src::qcommon::q_shared::byte,
    mut roq_flags: u16,
) {
    let mut i: isize = 0;
    let mut j: isize = 0;
    let mut two: isize = 0;
    let mut four: isize = 0;
    let mut aptr: *mut u16 = 0 as *mut u16;
    let mut bptr: *mut u16 = 0 as *mut u16;
    let mut cptr: *mut u16 = 0 as *mut u16;
    let mut dptr: *mut u16 = 0 as *mut u16;
    let mut y0: isize = 0;
    let mut y1: isize = 0;
    let mut y2: isize = 0;
    let mut y3: isize = 0;
    let mut cr: isize = 0;
    let mut cb: isize = 0;
    let mut bbptr: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut baptr: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut bcptr: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut bdptr: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut iaptr: C2RustUnnamed_14 = C2RustUnnamed_14 { i: 0 as *mut u32 };
    let mut ibptr: C2RustUnnamed_14 = C2RustUnnamed_14 { i: 0 as *mut u32 };
    let mut icptr: C2RustUnnamed_14 = C2RustUnnamed_14 { i: 0 as *mut u32 };
    let mut idptr: C2RustUnnamed_14 = C2RustUnnamed_14 { i: 0 as *mut u32 };
    if roq_flags == 0 {
        four = 256;
        two = four
    } else {
        two = (roq_flags as i32 >> 8) as isize;
        if two == 0 {
            two = 256
        }
        four = (roq_flags as i32 & 0xff) as isize
    }
    four *= 2;
    bptr = vq2.as_mut_ptr();
    if cinTable[currentHandle as usize].half as u64 == 0 {
        if cinTable[currentHandle as usize].smootheddouble as u64 == 0 {
            //
            // normal height
            //
            if cinTable[currentHandle as usize].samplesPerPixel == 2 {
                i = 0;
                while i < two {
                    let fresh4 = input;
                    input = input.offset(1);
                    y0 = *fresh4 as isize;
                    let fresh5 = input;
                    input = input.offset(1);
                    y1 = *fresh5 as isize;
                    let fresh6 = input;
                    input = input.offset(1);
                    y2 = *fresh6 as isize;
                    let fresh7 = input;
                    input = input.offset(1);
                    y3 = *fresh7 as isize;
                    let fresh8 = input;
                    input = input.offset(1);
                    cr = *fresh8 as isize;
                    let fresh9 = input;
                    input = input.offset(1);
                    cb = *fresh9 as isize;
                    let fresh10 = bptr;
                    bptr = bptr.offset(1);
                    *fresh10 = yuv_to_rgb(y0, cr, cb);
                    let fresh11 = bptr;
                    bptr = bptr.offset(1);
                    *fresh11 = yuv_to_rgb(y1, cr, cb);
                    let fresh12 = bptr;
                    bptr = bptr.offset(1);
                    *fresh12 = yuv_to_rgb(y2, cr, cb);
                    let fresh13 = bptr;
                    bptr = bptr.offset(1);
                    *fresh13 = yuv_to_rgb(y3, cr, cb);
                    i += 1
                }
                cptr = vq4.as_mut_ptr();
                dptr = vq8.as_mut_ptr();
                i = 0;
                while i < four {
                    let fresh14 = input;
                    input = input.offset(1);
                    aptr = vq2.as_mut_ptr().offset((*fresh14 as i32 * 4) as isize);
                    let fresh15 = input;
                    input = input.offset(1);
                    bptr = vq2.as_mut_ptr().offset((*fresh15 as i32 * 4) as isize);
                    j = 0;
                    while j < 2 {
                        let fresh16 = cptr;
                        cptr = cptr.offset(1);
                        *fresh16 = *aptr.offset(0);
                        let fresh17 = dptr;
                        dptr = dptr.offset(1);
                        *fresh17 = *aptr.offset(0);
                        let fresh18 = dptr;
                        dptr = dptr.offset(1);
                        *fresh18 = *aptr.offset(0);
                        let fresh19 = cptr;
                        cptr = cptr.offset(1);
                        *fresh19 = *aptr.offset(1);
                        let fresh20 = dptr;
                        dptr = dptr.offset(1);
                        *fresh20 = *aptr.offset(1);
                        let fresh21 = dptr;
                        dptr = dptr.offset(1);
                        *fresh21 = *aptr.offset(1);
                        let fresh22 = cptr;
                        cptr = cptr.offset(1);
                        *fresh22 = *bptr.offset(0);
                        let fresh23 = dptr;
                        dptr = dptr.offset(1);
                        *fresh23 = *bptr.offset(0);
                        let fresh24 = dptr;
                        dptr = dptr.offset(1);
                        *fresh24 = *bptr.offset(0);
                        let fresh25 = cptr;
                        cptr = cptr.offset(1);
                        *fresh25 = *bptr.offset(1);
                        let fresh26 = dptr;
                        dptr = dptr.offset(1);
                        *fresh26 = *bptr.offset(1);
                        let fresh27 = dptr;
                        dptr = dptr.offset(1);
                        *fresh27 = *bptr.offset(1);
                        let fresh28 = dptr;
                        dptr = dptr.offset(1);
                        *fresh28 = *aptr.offset(0);
                        let fresh29 = dptr;
                        dptr = dptr.offset(1);
                        *fresh29 = *aptr.offset(0);
                        let fresh30 = dptr;
                        dptr = dptr.offset(1);
                        *fresh30 = *aptr.offset(1);
                        let fresh31 = dptr;
                        dptr = dptr.offset(1);
                        *fresh31 = *aptr.offset(1);
                        let fresh32 = dptr;
                        dptr = dptr.offset(1);
                        *fresh32 = *bptr.offset(0);
                        let fresh33 = dptr;
                        dptr = dptr.offset(1);
                        *fresh33 = *bptr.offset(0);
                        let fresh34 = dptr;
                        dptr = dptr.offset(1);
                        *fresh34 = *bptr.offset(1);
                        let fresh35 = dptr;
                        dptr = dptr.offset(1);
                        *fresh35 = *bptr.offset(1);
                        aptr = aptr.offset(2);
                        bptr = bptr.offset(2);
                        j += 1
                    }
                    i += 1
                }
            } else if cinTable[currentHandle as usize].samplesPerPixel == 4 {
                ibptr.s = bptr;
                i = 0;
                while i < two {
                    let fresh36 = input;
                    input = input.offset(1);
                    y0 = *fresh36 as isize;
                    let fresh37 = input;
                    input = input.offset(1);
                    y1 = *fresh37 as isize;
                    let fresh38 = input;
                    input = input.offset(1);
                    y2 = *fresh38 as isize;
                    let fresh39 = input;
                    input = input.offset(1);
                    y3 = *fresh39 as isize;
                    let fresh40 = input;
                    input = input.offset(1);
                    cr = *fresh40 as isize;
                    let fresh41 = input;
                    input = input.offset(1);
                    cb = *fresh41 as isize;
                    let fresh42 = ibptr.i;
                    ibptr.i = ibptr.i.offset(1);
                    *fresh42 = yuv_to_rgb24(y0, cr, cb);
                    let fresh43 = ibptr.i;
                    ibptr.i = ibptr.i.offset(1);
                    *fresh43 = yuv_to_rgb24(y1, cr, cb);
                    let fresh44 = ibptr.i;
                    ibptr.i = ibptr.i.offset(1);
                    *fresh44 = yuv_to_rgb24(y2, cr, cb);
                    let fresh45 = ibptr.i;
                    ibptr.i = ibptr.i.offset(1);
                    *fresh45 = yuv_to_rgb24(y3, cr, cb);
                    i += 1
                }
                icptr.s = vq4.as_mut_ptr();
                idptr.s = vq8.as_mut_ptr();
                i = 0;
                while i < four {
                    iaptr.s = vq2.as_mut_ptr();
                    let fresh46 = input;
                    input = input.offset(1);
                    iaptr.i = iaptr.i.offset((*fresh46 as i32 * 4) as isize);
                    ibptr.s = vq2.as_mut_ptr();
                    let fresh47 = input;
                    input = input.offset(1);
                    ibptr.i = ibptr.i.offset((*fresh47 as i32 * 4) as isize);
                    j = 0;
                    while j < 2 {
                        let fresh48 = icptr.i;
                        icptr.i = icptr.i.offset(1);
                        *fresh48 = *iaptr.i.offset(0);
                        let fresh49 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh49 = *iaptr.i.offset(0);
                        let fresh50 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh50 = *iaptr.i.offset(0);
                        let fresh51 = icptr.i;
                        icptr.i = icptr.i.offset(1);
                        *fresh51 = *iaptr.i.offset(1);
                        let fresh52 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh52 = *iaptr.i.offset(1);
                        let fresh53 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh53 = *iaptr.i.offset(1);
                        let fresh54 = icptr.i;
                        icptr.i = icptr.i.offset(1);
                        *fresh54 = *ibptr.i.offset(0);
                        let fresh55 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh55 = *ibptr.i.offset(0);
                        let fresh56 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh56 = *ibptr.i.offset(0);
                        let fresh57 = icptr.i;
                        icptr.i = icptr.i.offset(1);
                        *fresh57 = *ibptr.i.offset(1);
                        let fresh58 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh58 = *ibptr.i.offset(1);
                        let fresh59 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh59 = *ibptr.i.offset(1);
                        let fresh60 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh60 = *iaptr.i.offset(0);
                        let fresh61 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh61 = *iaptr.i.offset(0);
                        let fresh62 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh62 = *iaptr.i.offset(1);
                        let fresh63 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh63 = *iaptr.i.offset(1);
                        let fresh64 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh64 = *ibptr.i.offset(0);
                        let fresh65 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh65 = *ibptr.i.offset(0);
                        let fresh66 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh66 = *ibptr.i.offset(1);
                        let fresh67 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh67 = *ibptr.i.offset(1);
                        iaptr.i = iaptr.i.offset(2);
                        ibptr.i = ibptr.i.offset(2);
                        j += 1
                    }
                    i += 1
                }
            } else if cinTable[currentHandle as usize].samplesPerPixel == 1 {
                bbptr = bptr as *mut crate::src::qcommon::q_shared::byte;
                i = 0;
                while i < two {
                    let fresh68 = input;
                    input = input.offset(1);
                    let fresh69 = bbptr;
                    bbptr = bbptr.offset(1);
                    *fresh69 = *cinTable[currentHandle as usize]
                        .gray
                        .offset(*fresh68 as isize);
                    let fresh70 = input;
                    input = input.offset(1);
                    let fresh71 = bbptr;
                    bbptr = bbptr.offset(1);
                    *fresh71 = *cinTable[currentHandle as usize]
                        .gray
                        .offset(*fresh70 as isize);
                    let fresh72 = input;
                    input = input.offset(1);
                    let fresh73 = bbptr;
                    bbptr = bbptr.offset(1);
                    *fresh73 = *cinTable[currentHandle as usize]
                        .gray
                        .offset(*fresh72 as isize);
                    let fresh74 = bbptr;
                    bbptr = bbptr.offset(1);
                    *fresh74 = *cinTable[currentHandle as usize]
                        .gray
                        .offset(*input as isize);
                    input = input.offset(3);
                    i += 1
                }
                bcptr = vq4.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte;
                bdptr = vq8.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte;
                i = 0;
                while i < four {
                    let fresh75 = input;
                    input = input.offset(1);
                    baptr = (vq2.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte)
                        .offset((*fresh75 as i32 * 4) as isize);
                    let fresh76 = input;
                    input = input.offset(1);
                    bbptr = (vq2.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte)
                        .offset((*fresh76 as i32 * 4) as isize);
                    j = 0;
                    while j < 2 {
                        let fresh77 = bcptr;
                        bcptr = bcptr.offset(1);
                        *fresh77 = *baptr.offset(0);
                        let fresh78 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh78 = *baptr.offset(0);
                        let fresh79 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh79 = *baptr.offset(0);
                        let fresh80 = bcptr;
                        bcptr = bcptr.offset(1);
                        *fresh80 = *baptr.offset(1);
                        let fresh81 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh81 = *baptr.offset(1);
                        let fresh82 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh82 = *baptr.offset(1);
                        let fresh83 = bcptr;
                        bcptr = bcptr.offset(1);
                        *fresh83 = *bbptr.offset(0);
                        let fresh84 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh84 = *bbptr.offset(0);
                        let fresh85 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh85 = *bbptr.offset(0);
                        let fresh86 = bcptr;
                        bcptr = bcptr.offset(1);
                        *fresh86 = *bbptr.offset(1);
                        let fresh87 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh87 = *bbptr.offset(1);
                        let fresh88 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh88 = *bbptr.offset(1);
                        let fresh89 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh89 = *baptr.offset(0);
                        let fresh90 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh90 = *baptr.offset(0);
                        let fresh91 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh91 = *baptr.offset(1);
                        let fresh92 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh92 = *baptr.offset(1);
                        let fresh93 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh93 = *bbptr.offset(0);
                        let fresh94 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh94 = *bbptr.offset(0);
                        let fresh95 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh95 = *bbptr.offset(1);
                        let fresh96 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh96 = *bbptr.offset(1);
                        baptr = baptr.offset(2);
                        bbptr = bbptr.offset(2);
                        j += 1
                    }
                    i += 1
                }
            }
        } else if cinTable[currentHandle as usize].samplesPerPixel == 2 {
            i = 0;
            while i < two {
                let fresh97 = input;
                input = input.offset(1);
                y0 = *fresh97 as isize;
                let fresh98 = input;
                input = input.offset(1);
                y1 = *fresh98 as isize;
                let fresh99 = input;
                input = input.offset(1);
                y2 = *fresh99 as isize;
                let fresh100 = input;
                input = input.offset(1);
                y3 = *fresh100 as isize;
                let fresh101 = input;
                input = input.offset(1);
                cr = *fresh101 as isize;
                let fresh102 = input;
                input = input.offset(1);
                cb = *fresh102 as isize;
                let fresh103 = bptr;
                bptr = bptr.offset(1);
                *fresh103 = yuv_to_rgb(y0, cr, cb);
                let fresh104 = bptr;
                bptr = bptr.offset(1);
                *fresh104 = yuv_to_rgb(y1, cr, cb);
                let fresh105 = bptr;
                bptr = bptr.offset(1);
                *fresh105 = yuv_to_rgb((y0 * 3 + y2) / 4, cr, cb);
                let fresh106 = bptr;
                bptr = bptr.offset(1);
                *fresh106 = yuv_to_rgb((y1 * 3 + y3) / 4, cr, cb);
                let fresh107 = bptr;
                bptr = bptr.offset(1);
                *fresh107 = yuv_to_rgb((y0 + y2 * 3) / 4, cr, cb);
                let fresh108 = bptr;
                bptr = bptr.offset(1);
                *fresh108 = yuv_to_rgb((y1 + y3 * 3) / 4, cr, cb);
                let fresh109 = bptr;
                bptr = bptr.offset(1);
                *fresh109 = yuv_to_rgb(y2, cr, cb);
                let fresh110 = bptr;
                bptr = bptr.offset(1);
                *fresh110 = yuv_to_rgb(y3, cr, cb);
                i += 1
            }
            cptr = vq4.as_mut_ptr();
            dptr = vq8.as_mut_ptr();
            i = 0;
            while i < four {
                let fresh111 = input;
                input = input.offset(1);
                aptr = vq2.as_mut_ptr().offset((*fresh111 as i32 * 8) as isize);
                let fresh112 = input;
                input = input.offset(1);
                bptr = vq2.as_mut_ptr().offset((*fresh112 as i32 * 8) as isize);
                j = 0;
                while j < 2 {
                    let fresh113 = cptr;
                    cptr = cptr.offset(1);
                    *fresh113 = *aptr.offset(0);
                    let fresh114 = dptr;
                    dptr = dptr.offset(1);
                    *fresh114 = *aptr.offset(0);
                    let fresh115 = dptr;
                    dptr = dptr.offset(1);
                    *fresh115 = *aptr.offset(0);
                    let fresh116 = cptr;
                    cptr = cptr.offset(1);
                    *fresh116 = *aptr.offset(1);
                    let fresh117 = dptr;
                    dptr = dptr.offset(1);
                    *fresh117 = *aptr.offset(1);
                    let fresh118 = dptr;
                    dptr = dptr.offset(1);
                    *fresh118 = *aptr.offset(1);
                    let fresh119 = cptr;
                    cptr = cptr.offset(1);
                    *fresh119 = *bptr.offset(0);
                    let fresh120 = dptr;
                    dptr = dptr.offset(1);
                    *fresh120 = *bptr.offset(0);
                    let fresh121 = dptr;
                    dptr = dptr.offset(1);
                    *fresh121 = *bptr.offset(0);
                    let fresh122 = cptr;
                    cptr = cptr.offset(1);
                    *fresh122 = *bptr.offset(1);
                    let fresh123 = dptr;
                    dptr = dptr.offset(1);
                    *fresh123 = *bptr.offset(1);
                    let fresh124 = dptr;
                    dptr = dptr.offset(1);
                    *fresh124 = *bptr.offset(1);
                    let fresh125 = dptr;
                    dptr = dptr.offset(1);
                    *fresh125 = *aptr.offset(0);
                    let fresh126 = dptr;
                    dptr = dptr.offset(1);
                    *fresh126 = *aptr.offset(0);
                    let fresh127 = dptr;
                    dptr = dptr.offset(1);
                    *fresh127 = *aptr.offset(1);
                    let fresh128 = dptr;
                    dptr = dptr.offset(1);
                    *fresh128 = *aptr.offset(1);
                    let fresh129 = dptr;
                    dptr = dptr.offset(1);
                    *fresh129 = *bptr.offset(0);
                    let fresh130 = dptr;
                    dptr = dptr.offset(1);
                    *fresh130 = *bptr.offset(0);
                    let fresh131 = dptr;
                    dptr = dptr.offset(1);
                    *fresh131 = *bptr.offset(1);
                    let fresh132 = dptr;
                    dptr = dptr.offset(1);
                    *fresh132 = *bptr.offset(1);
                    aptr = aptr.offset(2);
                    bptr = bptr.offset(2);
                    let fresh133 = cptr;
                    cptr = cptr.offset(1);
                    *fresh133 = *aptr.offset(0);
                    let fresh134 = dptr;
                    dptr = dptr.offset(1);
                    *fresh134 = *aptr.offset(0);
                    let fresh135 = dptr;
                    dptr = dptr.offset(1);
                    *fresh135 = *aptr.offset(0);
                    let fresh136 = cptr;
                    cptr = cptr.offset(1);
                    *fresh136 = *aptr.offset(1);
                    let fresh137 = dptr;
                    dptr = dptr.offset(1);
                    *fresh137 = *aptr.offset(1);
                    let fresh138 = dptr;
                    dptr = dptr.offset(1);
                    *fresh138 = *aptr.offset(1);
                    let fresh139 = cptr;
                    cptr = cptr.offset(1);
                    *fresh139 = *bptr.offset(0);
                    let fresh140 = dptr;
                    dptr = dptr.offset(1);
                    *fresh140 = *bptr.offset(0);
                    let fresh141 = dptr;
                    dptr = dptr.offset(1);
                    *fresh141 = *bptr.offset(0);
                    let fresh142 = cptr;
                    cptr = cptr.offset(1);
                    *fresh142 = *bptr.offset(1);
                    let fresh143 = dptr;
                    dptr = dptr.offset(1);
                    *fresh143 = *bptr.offset(1);
                    let fresh144 = dptr;
                    dptr = dptr.offset(1);
                    *fresh144 = *bptr.offset(1);
                    let fresh145 = dptr;
                    dptr = dptr.offset(1);
                    *fresh145 = *aptr.offset(0);
                    let fresh146 = dptr;
                    dptr = dptr.offset(1);
                    *fresh146 = *aptr.offset(0);
                    let fresh147 = dptr;
                    dptr = dptr.offset(1);
                    *fresh147 = *aptr.offset(1);
                    let fresh148 = dptr;
                    dptr = dptr.offset(1);
                    *fresh148 = *aptr.offset(1);
                    let fresh149 = dptr;
                    dptr = dptr.offset(1);
                    *fresh149 = *bptr.offset(0);
                    let fresh150 = dptr;
                    dptr = dptr.offset(1);
                    *fresh150 = *bptr.offset(0);
                    let fresh151 = dptr;
                    dptr = dptr.offset(1);
                    *fresh151 = *bptr.offset(1);
                    let fresh152 = dptr;
                    dptr = dptr.offset(1);
                    *fresh152 = *bptr.offset(1);
                    aptr = aptr.offset(2);
                    bptr = bptr.offset(2);
                    j += 1
                }
                i += 1
            }
        } else if cinTable[currentHandle as usize].samplesPerPixel == 4 {
            ibptr.s = bptr;
            i = 0;
            while i < two {
                let fresh153 = input;
                input = input.offset(1);
                y0 = *fresh153 as isize;
                let fresh154 = input;
                input = input.offset(1);
                y1 = *fresh154 as isize;
                let fresh155 = input;
                input = input.offset(1);
                y2 = *fresh155 as isize;
                let fresh156 = input;
                input = input.offset(1);
                y3 = *fresh156 as isize;
                let fresh157 = input;
                input = input.offset(1);
                cr = *fresh157 as isize;
                let fresh158 = input;
                input = input.offset(1);
                cb = *fresh158 as isize;
                let fresh159 = ibptr.i;
                ibptr.i = ibptr.i.offset(1);
                *fresh159 = yuv_to_rgb24(y0, cr, cb);
                let fresh160 = ibptr.i;
                ibptr.i = ibptr.i.offset(1);
                *fresh160 = yuv_to_rgb24(y1, cr, cb);
                let fresh161 = ibptr.i;
                ibptr.i = ibptr.i.offset(1);
                *fresh161 = yuv_to_rgb24((y0 * 3 + y2) / 4, cr, cb);
                let fresh162 = ibptr.i;
                ibptr.i = ibptr.i.offset(1);
                *fresh162 = yuv_to_rgb24((y1 * 3 + y3) / 4, cr, cb);
                let fresh163 = ibptr.i;
                ibptr.i = ibptr.i.offset(1);
                *fresh163 = yuv_to_rgb24((y0 + y2 * 3) / 4, cr, cb);
                let fresh164 = ibptr.i;
                ibptr.i = ibptr.i.offset(1);
                *fresh164 = yuv_to_rgb24((y1 + y3 * 3) / 4, cr, cb);
                let fresh165 = ibptr.i;
                ibptr.i = ibptr.i.offset(1);
                *fresh165 = yuv_to_rgb24(y2, cr, cb);
                let fresh166 = ibptr.i;
                ibptr.i = ibptr.i.offset(1);
                *fresh166 = yuv_to_rgb24(y3, cr, cb);
                i += 1
            }
            icptr.s = vq4.as_mut_ptr();
            idptr.s = vq8.as_mut_ptr();
            i = 0;
            while i < four {
                iaptr.s = vq2.as_mut_ptr();
                let fresh167 = input;
                input = input.offset(1);
                iaptr.i = iaptr.i.offset((*fresh167 as i32 * 8) as isize);
                ibptr.s = vq2.as_mut_ptr();
                let fresh168 = input;
                input = input.offset(1);
                ibptr.i = ibptr.i.offset((*fresh168 as i32 * 8) as isize);
                j = 0;
                while j < 2 {
                    let fresh169 = icptr.i;
                    icptr.i = icptr.i.offset(1);
                    *fresh169 = *iaptr.i.offset(0);
                    let fresh170 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh170 = *iaptr.i.offset(0);
                    let fresh171 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh171 = *iaptr.i.offset(0);
                    let fresh172 = icptr.i;
                    icptr.i = icptr.i.offset(1);
                    *fresh172 = *iaptr.i.offset(1);
                    let fresh173 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh173 = *iaptr.i.offset(1);
                    let fresh174 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh174 = *iaptr.i.offset(1);
                    let fresh175 = icptr.i;
                    icptr.i = icptr.i.offset(1);
                    *fresh175 = *ibptr.i.offset(0);
                    let fresh176 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh176 = *ibptr.i.offset(0);
                    let fresh177 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh177 = *ibptr.i.offset(0);
                    let fresh178 = icptr.i;
                    icptr.i = icptr.i.offset(1);
                    *fresh178 = *ibptr.i.offset(1);
                    let fresh179 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh179 = *ibptr.i.offset(1);
                    let fresh180 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh180 = *ibptr.i.offset(1);
                    let fresh181 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh181 = *iaptr.i.offset(0);
                    let fresh182 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh182 = *iaptr.i.offset(0);
                    let fresh183 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh183 = *iaptr.i.offset(1);
                    let fresh184 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh184 = *iaptr.i.offset(1);
                    let fresh185 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh185 = *ibptr.i.offset(0);
                    let fresh186 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh186 = *ibptr.i.offset(0);
                    let fresh187 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh187 = *ibptr.i.offset(1);
                    let fresh188 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh188 = *ibptr.i.offset(1);
                    iaptr.i = iaptr.i.offset(2);
                    ibptr.i = ibptr.i.offset(2);
                    let fresh189 = icptr.i;
                    icptr.i = icptr.i.offset(1);
                    *fresh189 = *iaptr.i.offset(0);
                    let fresh190 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh190 = *iaptr.i.offset(0);
                    let fresh191 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh191 = *iaptr.i.offset(0);
                    let fresh192 = icptr.i;
                    icptr.i = icptr.i.offset(1);
                    *fresh192 = *iaptr.i.offset(1);
                    let fresh193 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh193 = *iaptr.i.offset(1);
                    let fresh194 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh194 = *iaptr.i.offset(1);
                    let fresh195 = icptr.i;
                    icptr.i = icptr.i.offset(1);
                    *fresh195 = *ibptr.i.offset(0);
                    let fresh196 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh196 = *ibptr.i.offset(0);
                    let fresh197 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh197 = *ibptr.i.offset(0);
                    let fresh198 = icptr.i;
                    icptr.i = icptr.i.offset(1);
                    *fresh198 = *ibptr.i.offset(1);
                    let fresh199 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh199 = *ibptr.i.offset(1);
                    let fresh200 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh200 = *ibptr.i.offset(1);
                    let fresh201 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh201 = *iaptr.i.offset(0);
                    let fresh202 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh202 = *iaptr.i.offset(0);
                    let fresh203 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh203 = *iaptr.i.offset(1);
                    let fresh204 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh204 = *iaptr.i.offset(1);
                    let fresh205 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh205 = *ibptr.i.offset(0);
                    let fresh206 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh206 = *ibptr.i.offset(0);
                    let fresh207 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh207 = *ibptr.i.offset(1);
                    let fresh208 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh208 = *ibptr.i.offset(1);
                    iaptr.i = iaptr.i.offset(2);
                    ibptr.i = ibptr.i.offset(2);
                    j += 1
                }
                i += 1
            }
        } else if cinTable[currentHandle as usize].samplesPerPixel == 1 {
            bbptr = bptr as *mut crate::src::qcommon::q_shared::byte;
            i = 0;
            while i < two {
                let fresh209 = input;
                input = input.offset(1);
                y0 = *fresh209 as isize;
                let fresh210 = input;
                input = input.offset(1);
                y1 = *fresh210 as isize;
                let fresh211 = input;
                input = input.offset(1);
                y2 = *fresh211 as isize;
                y3 = *input as isize;
                input = input.offset(3);
                let fresh212 = bbptr;
                bbptr = bbptr.offset(1);
                *fresh212 = *cinTable[currentHandle as usize].gray.offset(y0);
                let fresh213 = bbptr;
                bbptr = bbptr.offset(1);
                *fresh213 = *cinTable[currentHandle as usize].gray.offset(y1);
                let fresh214 = bbptr;
                bbptr = bbptr.offset(1);
                *fresh214 = *cinTable[currentHandle as usize]
                    .gray
                    .offset((y0 * 3 + y2) / 4);
                let fresh215 = bbptr;
                bbptr = bbptr.offset(1);
                *fresh215 = *cinTable[currentHandle as usize]
                    .gray
                    .offset((y1 * 3 + y3) / 4);
                let fresh216 = bbptr;
                bbptr = bbptr.offset(1);
                *fresh216 = *cinTable[currentHandle as usize]
                    .gray
                    .offset((y0 + y2 * 3) / 4);
                let fresh217 = bbptr;
                bbptr = bbptr.offset(1);
                *fresh217 = *cinTable[currentHandle as usize]
                    .gray
                    .offset((y1 + y3 * 3) / 4);
                let fresh218 = bbptr;
                bbptr = bbptr.offset(1);
                *fresh218 = *cinTable[currentHandle as usize].gray.offset(y2);
                let fresh219 = bbptr;
                bbptr = bbptr.offset(1);
                *fresh219 = *cinTable[currentHandle as usize].gray.offset(y3);
                i += 1
            }
            bcptr = vq4.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte;
            bdptr = vq8.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte;
            i = 0;
            while i < four {
                let fresh220 = input;
                input = input.offset(1);
                baptr = (vq2.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte)
                    .offset((*fresh220 as i32 * 8) as isize);
                let fresh221 = input;
                input = input.offset(1);
                bbptr = (vq2.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte)
                    .offset((*fresh221 as i32 * 8) as isize);
                j = 0;
                while j < 2 {
                    let fresh222 = bcptr;
                    bcptr = bcptr.offset(1);
                    *fresh222 = *baptr.offset(0);
                    let fresh223 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh223 = *baptr.offset(0);
                    let fresh224 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh224 = *baptr.offset(0);
                    let fresh225 = bcptr;
                    bcptr = bcptr.offset(1);
                    *fresh225 = *baptr.offset(1);
                    let fresh226 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh226 = *baptr.offset(1);
                    let fresh227 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh227 = *baptr.offset(1);
                    let fresh228 = bcptr;
                    bcptr = bcptr.offset(1);
                    *fresh228 = *bbptr.offset(0);
                    let fresh229 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh229 = *bbptr.offset(0);
                    let fresh230 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh230 = *bbptr.offset(0);
                    let fresh231 = bcptr;
                    bcptr = bcptr.offset(1);
                    *fresh231 = *bbptr.offset(1);
                    let fresh232 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh232 = *bbptr.offset(1);
                    let fresh233 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh233 = *bbptr.offset(1);
                    let fresh234 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh234 = *baptr.offset(0);
                    let fresh235 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh235 = *baptr.offset(0);
                    let fresh236 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh236 = *baptr.offset(1);
                    let fresh237 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh237 = *baptr.offset(1);
                    let fresh238 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh238 = *bbptr.offset(0);
                    let fresh239 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh239 = *bbptr.offset(0);
                    let fresh240 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh240 = *bbptr.offset(1);
                    let fresh241 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh241 = *bbptr.offset(1);
                    baptr = baptr.offset(2);
                    bbptr = bbptr.offset(2);
                    let fresh242 = bcptr;
                    bcptr = bcptr.offset(1);
                    *fresh242 = *baptr.offset(0);
                    let fresh243 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh243 = *baptr.offset(0);
                    let fresh244 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh244 = *baptr.offset(0);
                    let fresh245 = bcptr;
                    bcptr = bcptr.offset(1);
                    *fresh245 = *baptr.offset(1);
                    let fresh246 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh246 = *baptr.offset(1);
                    let fresh247 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh247 = *baptr.offset(1);
                    let fresh248 = bcptr;
                    bcptr = bcptr.offset(1);
                    *fresh248 = *bbptr.offset(0);
                    let fresh249 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh249 = *bbptr.offset(0);
                    let fresh250 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh250 = *bbptr.offset(0);
                    let fresh251 = bcptr;
                    bcptr = bcptr.offset(1);
                    *fresh251 = *bbptr.offset(1);
                    let fresh252 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh252 = *bbptr.offset(1);
                    let fresh253 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh253 = *bbptr.offset(1);
                    let fresh254 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh254 = *baptr.offset(0);
                    let fresh255 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh255 = *baptr.offset(0);
                    let fresh256 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh256 = *baptr.offset(1);
                    let fresh257 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh257 = *baptr.offset(1);
                    let fresh258 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh258 = *bbptr.offset(0);
                    let fresh259 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh259 = *bbptr.offset(0);
                    let fresh260 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh260 = *bbptr.offset(1);
                    let fresh261 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh261 = *bbptr.offset(1);
                    baptr = baptr.offset(2);
                    bbptr = bbptr.offset(2);
                    j += 1
                }
                i += 1
            }
        }
    } else if cinTable[currentHandle as usize].samplesPerPixel == 2 {
        i = 0;
        while i < two {
            y0 = *input as isize;
            input = input.offset(2);
            y2 = *input as isize;
            input = input.offset(2);
            let fresh262 = input;
            input = input.offset(1);
            cr = *fresh262 as isize;
            let fresh263 = input;
            input = input.offset(1);
            cb = *fresh263 as isize;
            let fresh264 = bptr;
            bptr = bptr.offset(1);
            *fresh264 = yuv_to_rgb(y0, cr, cb);
            let fresh265 = bptr;
            bptr = bptr.offset(1);
            *fresh265 = yuv_to_rgb(y2, cr, cb);
            i += 1
        }
        cptr = vq4.as_mut_ptr();
        dptr = vq8.as_mut_ptr();
        i = 0;
        while i < four {
            let fresh266 = input;
            input = input.offset(1);
            aptr = vq2.as_mut_ptr().offset((*fresh266 as i32 * 2) as isize);
            let fresh267 = input;
            input = input.offset(1);
            bptr = vq2.as_mut_ptr().offset((*fresh267 as i32 * 2) as isize);
            j = 0;
            while j < 2 {
                let fresh268 = cptr;
                cptr = cptr.offset(1);
                *fresh268 = *aptr;
                let fresh269 = dptr;
                dptr = dptr.offset(1);
                *fresh269 = *aptr;
                let fresh270 = dptr;
                dptr = dptr.offset(1);
                *fresh270 = *aptr;
                let fresh271 = cptr;
                cptr = cptr.offset(1);
                *fresh271 = *bptr;
                let fresh272 = dptr;
                dptr = dptr.offset(1);
                *fresh272 = *bptr;
                let fresh273 = dptr;
                dptr = dptr.offset(1);
                *fresh273 = *bptr;
                let fresh274 = dptr;
                dptr = dptr.offset(1);
                *fresh274 = *aptr;
                let fresh275 = dptr;
                dptr = dptr.offset(1);
                *fresh275 = *aptr;
                let fresh276 = dptr;
                dptr = dptr.offset(1);
                *fresh276 = *bptr;
                let fresh277 = dptr;
                dptr = dptr.offset(1);
                *fresh277 = *bptr;
                aptr = aptr.offset(1);
                bptr = bptr.offset(1);
                j += 1
            }
            i += 1
        }
    } else if cinTable[currentHandle as usize].samplesPerPixel == 1 {
        bbptr = bptr as *mut crate::src::qcommon::q_shared::byte;
        i = 0;
        while i < two {
            let fresh278 = bbptr;
            bbptr = bbptr.offset(1);
            *fresh278 = *cinTable[currentHandle as usize]
                .gray
                .offset(*input as isize);
            input = input.offset(2);
            let fresh279 = bbptr;
            bbptr = bbptr.offset(1);
            *fresh279 = *cinTable[currentHandle as usize]
                .gray
                .offset(*input as isize);
            input = input.offset(4);
            i += 1
        }
        bcptr = vq4.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte;
        bdptr = vq8.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte;
        i = 0;
        while i < four {
            let fresh280 = input;
            input = input.offset(1);
            baptr = (vq2.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte)
                .offset((*fresh280 as i32 * 2) as isize);
            let fresh281 = input;
            input = input.offset(1);
            bbptr = (vq2.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte)
                .offset((*fresh281 as i32 * 2) as isize);
            j = 0;
            while j < 2 {
                let fresh282 = bcptr;
                bcptr = bcptr.offset(1);
                *fresh282 = *baptr;
                let fresh283 = bdptr;
                bdptr = bdptr.offset(1);
                *fresh283 = *baptr;
                let fresh284 = bdptr;
                bdptr = bdptr.offset(1);
                *fresh284 = *baptr;
                let fresh285 = bcptr;
                bcptr = bcptr.offset(1);
                *fresh285 = *bbptr;
                let fresh286 = bdptr;
                bdptr = bdptr.offset(1);
                *fresh286 = *bbptr;
                let fresh287 = bdptr;
                bdptr = bdptr.offset(1);
                *fresh287 = *bbptr;
                let fresh288 = bdptr;
                bdptr = bdptr.offset(1);
                *fresh288 = *baptr;
                let fresh289 = bdptr;
                bdptr = bdptr.offset(1);
                *fresh289 = *baptr;
                let fresh290 = bdptr;
                bdptr = bdptr.offset(1);
                *fresh290 = *bbptr;
                let fresh291 = bdptr;
                bdptr = bdptr.offset(1);
                *fresh291 = *bbptr;
                baptr = baptr.offset(1);
                bbptr = bbptr.offset(1);
                j += 1
            }
            i += 1
        }
    } else if cinTable[currentHandle as usize].samplesPerPixel == 4 {
        ibptr.s = bptr;
        i = 0;
        while i < two {
            y0 = *input as isize;
            input = input.offset(2);
            y2 = *input as isize;
            input = input.offset(2);
            let fresh292 = input;
            input = input.offset(1);
            cr = *fresh292 as isize;
            let fresh293 = input;
            input = input.offset(1);
            cb = *fresh293 as isize;
            let fresh294 = ibptr.i;
            ibptr.i = ibptr.i.offset(1);
            *fresh294 = yuv_to_rgb24(y0, cr, cb);
            let fresh295 = ibptr.i;
            ibptr.i = ibptr.i.offset(1);
            *fresh295 = yuv_to_rgb24(y2, cr, cb);
            i += 1
        }
        icptr.s = vq4.as_mut_ptr();
        idptr.s = vq8.as_mut_ptr();
        i = 0;
        while i < four {
            iaptr.s = vq2.as_mut_ptr();
            let fresh296 = input;
            input = input.offset(1);
            iaptr.i = iaptr.i.offset((*fresh296 as i32 * 2) as isize);
            let fresh297 = input;
            input = input.offset(1);
            ibptr.s = vq2.as_mut_ptr().offset((*fresh297 as i32 * 2) as isize);
            let fresh298 = input;
            input = input.offset(1);
            ibptr.i = ibptr.i.offset((*fresh298 as i32 * 2) as isize);
            j = 0;
            while j < 2 {
                let fresh299 = icptr.i;
                icptr.i = icptr.i.offset(1);
                *fresh299 = *iaptr.i;
                let fresh300 = idptr.i;
                idptr.i = idptr.i.offset(1);
                *fresh300 = *iaptr.i;
                let fresh301 = idptr.i;
                idptr.i = idptr.i.offset(1);
                *fresh301 = *iaptr.i;
                let fresh302 = icptr.i;
                icptr.i = icptr.i.offset(1);
                *fresh302 = *ibptr.i;
                let fresh303 = idptr.i;
                idptr.i = idptr.i.offset(1);
                *fresh303 = *ibptr.i;
                let fresh304 = idptr.i;
                idptr.i = idptr.i.offset(1);
                *fresh304 = *ibptr.i;
                let fresh305 = idptr.i;
                idptr.i = idptr.i.offset(1);
                *fresh305 = *iaptr.i;
                let fresh306 = idptr.i;
                idptr.i = idptr.i.offset(1);
                *fresh306 = *iaptr.i;
                let fresh307 = idptr.i;
                idptr.i = idptr.i.offset(1);
                *fresh307 = *ibptr.i;
                let fresh308 = idptr.i;
                idptr.i = idptr.i.offset(1);
                *fresh308 = *ibptr.i;
                iaptr.i = iaptr.i.offset(1);
                ibptr.i = ibptr.i.offset(1);
                j += 1
            }
            i += 1
        }
    };
}
//
// double height, smoothed
//
//
// 1/4 screen
//
/* *****************************************************************************
*
* Function:
*
* Description:
*
******************************************************************************/

unsafe extern "C" fn recurseQuad(
    mut startX: isize,
    mut startY: isize,
    mut quadSize: isize,
    mut xOff: isize,
    mut yOff: isize,
) {
    let mut scroff: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut bigx: isize = 0;
    let mut bigy: isize = 0;
    let mut lowx: isize = 0;
    let mut lowy: isize = 0;
    let mut useY: isize = 0;
    let mut offset: isize = 0;
    offset = cinTable[currentHandle as usize].screenDelta;
    lowy = 0;
    lowx = lowy;
    bigx = cinTable[currentHandle as usize].xsize as isize;
    bigy = cinTable[currentHandle as usize].ysize as isize;
    if bigx > cinTable[currentHandle as usize].CIN_WIDTH as isize {
        bigx = cinTable[currentHandle as usize].CIN_WIDTH as isize
    }
    if bigy > cinTable[currentHandle as usize].CIN_HEIGHT as isize {
        bigy = cinTable[currentHandle as usize].CIN_HEIGHT as isize
    }
    if startX >= lowx
        && startX + quadSize <= bigx
        && startY + quadSize <= bigy
        && startY >= lowy
        && quadSize <= 8isize
    {
        useY = startY;
        scroff = cin
            .linbuf
            .as_mut_ptr()
            .offset(
                (useY + (cinTable[currentHandle as usize].CIN_HEIGHT as isize - bigy >> 1) + yOff)
                    * cinTable[currentHandle as usize].samplesPerLine,
            )
            .offset((startX + xOff) * cinTable[currentHandle as usize].samplesPerPixel);
        cin.qStatus[0][cinTable[currentHandle as usize].onQuad as usize] = scroff;
        let fresh309 = cinTable[currentHandle as usize].onQuad;
        cinTable[currentHandle as usize].onQuad = cinTable[currentHandle as usize].onQuad + 1;
        cin.qStatus[1][fresh309 as usize] = scroff.offset(offset)
    }
    if quadSize != 4isize {
        quadSize >>= 1isize;
        recurseQuad(startX, startY, quadSize, xOff, yOff);
        recurseQuad(startX + quadSize, startY, quadSize, xOff, yOff);
        recurseQuad(startX, startY + quadSize, quadSize, xOff, yOff);
        recurseQuad(startX + quadSize, startY + quadSize, quadSize, xOff, yOff);
    };
}
/* *****************************************************************************
*
* Function:
*
* Description:
*
******************************************************************************/

unsafe extern "C" fn setupQuad(mut xOff: isize, mut yOff: isize) {
    let mut numQuadCels: isize = 0; // for overflow
    let mut i: isize = 0; // eoq
    let mut _x: isize = 0;
    let mut _y: isize = 0;
    let mut temp: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    if xOff == cin.oldXOff
        && yOff == cin.oldYOff
        && cinTable[currentHandle as usize].ysize as isize == cin.oldysize
        && cinTable[currentHandle as usize].xsize as isize == cin.oldxsize
    {
        return;
    }
    cin.oldXOff = xOff;
    cin.oldYOff = yOff;
    cin.oldysize = cinTable[currentHandle as usize].ysize as isize;
    cin.oldxsize = cinTable[currentHandle as usize].xsize as isize;
    numQuadCels = cinTable[currentHandle as usize]
        .xsize
        .wrapping_mul(cinTable[currentHandle as usize].ysize)
        .wrapping_div(16u32) as isize;
    numQuadCels += numQuadCels / 4;
    numQuadCels += 64;
    cinTable[currentHandle as usize].onQuad = 0;

    for y in (0..cinTable[currentHandle as usize].ysize as isize).step_by(16 as usize) {
        for x in (0..cinTable[currentHandle as usize].xsize as isize).step_by(16 as usize) {
            recurseQuad(x, y, 16, xOff, yOff);
        }
    }
    temp = 0 as *mut crate::src::qcommon::q_shared::byte;
    i = numQuadCels - 64;
    while i < numQuadCels {
        cin.qStatus[0][i as usize] = temp;
        cin.qStatus[1][i as usize] = temp;
        i += 1
        // eoq
    }
}
/* *****************************************************************************
*
* Function:
*
* Description:
*
******************************************************************************/

unsafe extern "C" fn readQuadInfo(mut qData: *mut crate::src::qcommon::q_shared::byte) {
    if currentHandle < 0 {
        return;
    }
    cinTable[currentHandle as usize].xsize =
        (*qData.offset(0) as i32 + *qData.offset(1) as i32 * 256) as u32;
    cinTable[currentHandle as usize].ysize =
        (*qData.offset(2) as i32 + *qData.offset(3) as i32 * 256) as u32;
    cinTable[currentHandle as usize].maxsize =
        (*qData.offset(4) as i32 + *qData.offset(5) as i32 * 256) as u32;
    cinTable[currentHandle as usize].minsize =
        (*qData.offset(6) as i32 + *qData.offset(7) as i32 * 256) as u32;
    cinTable[currentHandle as usize].CIN_HEIGHT = cinTable[currentHandle as usize].ysize as i32;
    cinTable[currentHandle as usize].CIN_WIDTH = cinTable[currentHandle as usize].xsize as i32;
    cinTable[currentHandle as usize].samplesPerLine = cinTable[currentHandle as usize].CIN_WIDTH
        as isize
        * cinTable[currentHandle as usize].samplesPerPixel;
    cinTable[currentHandle as usize].screenDelta = cinTable[currentHandle as usize].CIN_HEIGHT
        as isize
        * cinTable[currentHandle as usize].samplesPerLine;
    cinTable[currentHandle as usize].half = crate::src::qcommon::q_shared::qfalse;
    cinTable[currentHandle as usize].smootheddouble = crate::src::qcommon::q_shared::qfalse;
    cinTable[currentHandle as usize].VQ0 = cinTable[currentHandle as usize].VQNormal;
    cinTable[currentHandle as usize].VQ1 = cinTable[currentHandle as usize].VQBuffer;
    cinTable[currentHandle as usize].t[0] = cinTable[currentHandle as usize].screenDelta;
    cinTable[currentHandle as usize].t[1] = -cinTable[currentHandle as usize].screenDelta;
    cinTable[currentHandle as usize].drawX = cinTable[currentHandle as usize].CIN_WIDTH as isize;
    cinTable[currentHandle as usize].drawY = cinTable[currentHandle as usize].CIN_HEIGHT as isize;
    // rage pro is very slow at 512 wide textures, voodoo can't do it at all
    if crate::src::client::cl_main::cls.glconfig.hardwareType == crate::tr_types_h::GLHW_RAGEPRO
        || crate::src::client::cl_main::cls.glconfig.maxTextureSize <= 256
    {
        if cinTable[currentHandle as usize].drawX > 256 {
            cinTable[currentHandle as usize].drawX = 256
        }
        if cinTable[currentHandle as usize].drawY > 256 {
            cinTable[currentHandle as usize].drawY = 256
        }
        if cinTable[currentHandle as usize].CIN_WIDTH != 256
            || cinTable[currentHandle as usize].CIN_HEIGHT != 256
        {
            crate::src::qcommon::common::Com_Printf(
                b"HACK: approxmimating cinematic for Rage Pro or Voodoo\n\x00" as *const u8
                    as *const i8,
            );
        }
    };
}
/* *****************************************************************************
*
* Function:
*
* Description:
*
******************************************************************************/

unsafe extern "C" fn RoQPrepMcomp(mut xoff: isize, mut yoff: isize) {
    let mut i: isize = 0;
    let mut j: isize = 0;
    let mut _x: isize = 0;
    let mut y: isize = 0;
    let mut temp: isize = 0;
    let mut temp2: isize = 0;
    i = cinTable[currentHandle as usize].samplesPerLine;
    j = cinTable[currentHandle as usize].samplesPerPixel;
    if cinTable[currentHandle as usize].xsize
        == cinTable[currentHandle as usize].ysize.wrapping_mul(4u32)
        && cinTable[currentHandle as usize].half as u64 == 0
    {
        j = j + j;
        i = i + i
    }
    y = 0;
    while y < 16 {
        temp2 = (y + yoff - 8) * i;

        for x in 0..16 {
            temp = (x + xoff - 8) * j;

            cin.mcomp[(x * 16 + y) as usize] =
                (cinTable[currentHandle as usize].normalBuffer0 - (temp2 + temp)) as i32;
        }
        y += 1
    }
}
/* *****************************************************************************
*
* Function:
*
* Description:
*
******************************************************************************/

unsafe extern "C" fn initRoQ() {
    if currentHandle < 0 {
        return;
    }
    cinTable[currentHandle as usize].VQNormal = ::std::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                _: *mut *mut crate::src::qcommon::q_shared::byte,
                _: *mut u8,
            ) -> (),
        >,
        Option<
            unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::byte,
                _: *mut libc::c_void,
            ) -> (),
        >,
    >(Some(
        blitVQQuad32fs
            as unsafe extern "C" fn(
                _: *mut *mut crate::src::qcommon::q_shared::byte,
                _: *mut u8,
            ) -> (),
    ));
    cinTable[currentHandle as usize].VQBuffer = ::std::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                _: *mut *mut crate::src::qcommon::q_shared::byte,
                _: *mut u8,
            ) -> (),
        >,
        Option<
            unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::byte,
                _: *mut libc::c_void,
            ) -> (),
        >,
    >(Some(
        blitVQQuad32fs
            as unsafe extern "C" fn(
                _: *mut *mut crate::src::qcommon::q_shared::byte,
                _: *mut u8,
            ) -> (),
    ));
    cinTable[currentHandle as usize].samplesPerPixel = 4;
    ROQ_GenYUVTables();
    RllSetupTable();
}
/* *****************************************************************************
*
* Function:
*
* Description:
*
******************************************************************************/
/*
static byte* RoQFetchInterlaced( byte *source ) {
    int x, *src, *dst;

    if (currentHandle < 0) return NULL;

    src = (int *)source;
    dst = (int *)cinTable[currentHandle].buf2;

    for(x=0;x<256*256;x++) {
        *dst = *src;
        dst++; src += 2;
    }
    return cinTable[currentHandle].buf2;
}
*/

unsafe extern "C" fn RoQReset() {
    if currentHandle < 0 {
        return;
    }
    crate::src::qcommon::files::FS_FCloseFile(cinTable[currentHandle as usize].iFile);
    crate::src::qcommon::files::FS_FOpenFileRead(
        cinTable[currentHandle as usize].fileName.as_mut_ptr(),
        &mut (*cinTable.as_mut_ptr().offset(currentHandle as isize)).iFile,
        crate::src::qcommon::q_shared::qtrue,
    );
    // let the background thread start reading ahead
    crate::src::qcommon::files::FS_Read(
        cin.file.as_mut_ptr() as *mut libc::c_void,
        16,
        cinTable[currentHandle as usize].iFile,
    );
    RoQ_init();
    cinTable[currentHandle as usize].status = crate::src::qcommon::q_shared::FMV_LOOPED;
}
/* *****************************************************************************
*
* Function:
*
* Description:
*
******************************************************************************/

unsafe extern "C" fn RoQInterrupt() {
    let mut framedata: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut sbuf: [i16; 32768] = [0; 32768];
    let mut ssize: i32 = 0;
    if currentHandle < 0 {
        return;
    }
    crate::src::qcommon::files::FS_Read(
        cin.file.as_mut_ptr() as *mut libc::c_void,
        cinTable[currentHandle as usize]
            .RoQFrameSize
            .wrapping_add(8u32) as i32,
        cinTable[currentHandle as usize].iFile,
    );
    if cinTable[currentHandle as usize].RoQPlayed >= cinTable[currentHandle as usize].ROQSize {
        if cinTable[currentHandle as usize].holdAtEnd == crate::src::qcommon::q_shared::qfalse {
            if cinTable[currentHandle as usize].looping as u64 != 0 {
                RoQReset();
            } else {
                cinTable[currentHandle as usize].status = crate::src::qcommon::q_shared::FMV_EOF
            }
        } else {
            cinTable[currentHandle as usize].status = crate::src::qcommon::q_shared::FMV_IDLE
        }
        return;
    }
    framedata = cin.file.as_mut_ptr();
    loop
    //
    // new frame is ready
    //
    {
        match cinTable[currentHandle as usize].roq_id {
            4113 => {
                if cinTable[currentHandle as usize].numQuads & 1 != 0 {
                    cinTable[currentHandle as usize].normalBuffer0 =
                        cinTable[currentHandle as usize].t[1];
                    RoQPrepMcomp(
                        cinTable[currentHandle as usize].roqF0,
                        cinTable[currentHandle as usize].roqF1,
                    );
                    cinTable[currentHandle as usize]
                        .VQ1
                        .expect("non-null function pointer")(
                        cin.qStatus[1].as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte,
                        framedata as *mut libc::c_void,
                    );
                    cinTable[currentHandle as usize].buf = cin
                        .linbuf
                        .as_mut_ptr()
                        .offset(cinTable[currentHandle as usize].screenDelta)
                } else {
                    cinTable[currentHandle as usize].normalBuffer0 =
                        cinTable[currentHandle as usize].t[0];
                    RoQPrepMcomp(
                        cinTable[currentHandle as usize].roqF0,
                        cinTable[currentHandle as usize].roqF1,
                    );
                    cinTable[currentHandle as usize]
                        .VQ0
                        .expect("non-null function pointer")(
                        cin.qStatus[0].as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte,
                        framedata as *mut libc::c_void,
                    );
                    cinTable[currentHandle as usize].buf = cin.linbuf.as_mut_ptr()
                }
                if cinTable[currentHandle as usize].numQuads == 0 {
                    // first frame
                    crate::stdlib::memcpy(
                        cin.linbuf
                            .as_mut_ptr()
                            .offset(cinTable[currentHandle as usize].screenDelta)
                            as *mut libc::c_void,
                        cin.linbuf.as_mut_ptr() as *const libc::c_void,
                        (cinTable[currentHandle as usize].samplesPerLine
                            * cinTable[currentHandle as usize].ysize as isize)
                            as usize,
                    ); // for header
                }
                cinTable[currentHandle as usize].numQuads += 1;
                cinTable[currentHandle as usize].dirty = crate::src::qcommon::q_shared::qtrue
            }
            4098 => {
                decodeCodeBook(framedata, cinTable[currentHandle as usize].roq_flags as u16);
            }
            4128 => {
                if cinTable[currentHandle as usize].silent as u64 == 0 {
                    ssize = RllDecodeMonoToStereo(
                        framedata,
                        sbuf.as_mut_ptr(),
                        cinTable[currentHandle as usize].RoQFrameSize,
                        0,
                        cinTable[currentHandle as usize].roq_flags as u16,
                    ) as i32;
                    crate::src::client::snd_main::S_RawSamples(
                        0i32,
                        ssize,
                        22050i32,
                        2i32,
                        1i32,
                        sbuf.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte,
                        1.0f32,
                        -(1i32),
                    );
                }
            }
            4129 => {
                if cinTable[currentHandle as usize].silent as u64 == 0 {
                    if cinTable[currentHandle as usize].numQuads == -1 {
                        crate::src::client::snd_main::S_Update();
                        crate::src::client::snd_dma::s_rawend[0] = s_soundtime
                    }
                    ssize = RllDecodeStereoToStereo(
                        framedata,
                        sbuf.as_mut_ptr(),
                        cinTable[currentHandle as usize].RoQFrameSize,
                        0,
                        cinTable[currentHandle as usize].roq_flags as u16,
                    ) as i32;
                    crate::src::client::snd_main::S_RawSamples(
                        0i32,
                        ssize,
                        22050i32,
                        2i32,
                        2i32,
                        sbuf.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte,
                        1.0f32,
                        -(1i32),
                    );
                }
            }
            4097 => {
                if cinTable[currentHandle as usize].numQuads == -1 {
                    readQuadInfo(framedata);
                    setupQuad(0, 0);
                    cinTable[currentHandle as usize].lastTime = CL_ScaledMilliseconds();
                    cinTable[currentHandle as usize].startTime =
                        cinTable[currentHandle as usize].lastTime
                }
                if cinTable[currentHandle as usize].numQuads != 1 {
                    cinTable[currentHandle as usize].numQuads = 0
                }
            }
            4144 => {
                cinTable[currentHandle as usize].inMemory = cinTable[currentHandle as usize]
                    .roq_flags
                    as crate::src::qcommon::q_shared::qboolean;
                cinTable[currentHandle as usize].RoQFrameSize = 0
            }
            4115 => cinTable[currentHandle as usize].RoQFrameSize = 0,
            4114 => {}
            _ => cinTable[currentHandle as usize].status = crate::src::qcommon::q_shared::FMV_EOF,
        }
        //
        // read in next frame data
        //
        if cinTable[currentHandle as usize].RoQPlayed >= cinTable[currentHandle as usize].ROQSize {
            if cinTable[currentHandle as usize].holdAtEnd == crate::src::qcommon::q_shared::qfalse {
                if cinTable[currentHandle as usize].looping as u64 != 0 {
                    RoQReset();
                } else {
                    cinTable[currentHandle as usize].status = crate::src::qcommon::q_shared::FMV_EOF
                }
            } else {
                cinTable[currentHandle as usize].status = crate::src::qcommon::q_shared::FMV_IDLE
            }
            return;
        }
        framedata = framedata.offset(cinTable[currentHandle as usize].RoQFrameSize as isize);
        cinTable[currentHandle as usize].roq_id =
            (*framedata.offset(0) as i32 + *framedata.offset(1) as i32 * 256) as u32;
        cinTable[currentHandle as usize].RoQFrameSize = (*framedata.offset(2) as i32
            + *framedata.offset(3) as i32 * 256
            + *framedata.offset(4) as i32 * 65536)
            as u32;
        cinTable[currentHandle as usize].roq_flags =
            (*framedata.offset(6) as i32 + *framedata.offset(7) as i32 * 256) as isize;
        cinTable[currentHandle as usize].roqF0 = *framedata.offset(7) as i8 as isize;
        cinTable[currentHandle as usize].roqF1 = *framedata.offset(6) as i8 as isize;
        if cinTable[currentHandle as usize].RoQFrameSize > 65536
            || cinTable[currentHandle as usize].roq_id == 0x1084
        {
            crate::src::qcommon::common::Com_DPrintf(
                b"roq_size>65536||roq_id==0x1084\n\x00" as *const u8 as *const i8,
            );
            cinTable[currentHandle as usize].status = crate::src::qcommon::q_shared::FMV_EOF;
            if cinTable[currentHandle as usize].looping as u64 != 0 {
                RoQReset();
            }
            return;
        }
        if !(cinTable[currentHandle as usize].inMemory != 0
            && cinTable[currentHandle as usize].status != crate::src::qcommon::q_shared::FMV_EOF)
        {
            break;
        }
        cinTable[currentHandle as usize].inMemory -= 1;
        framedata = framedata.offset(8)
    }
    //
    // one more frame hits the dust
    //
    //	assert(cinTable[currentHandle].RoQFrameSize <= 65536);
    //	r = FS_Read( cin.file, cinTable[currentHandle].RoQFrameSize+8, cinTable[currentHandle].iFile );
    cinTable[currentHandle as usize].RoQPlayed += cinTable[currentHandle as usize]
        .RoQFrameSize
        .wrapping_add(8u32) as isize;
}
/* *****************************************************************************
*
* Function:
*
* Description:
*
******************************************************************************/

unsafe extern "C" fn RoQ_init() {
    cinTable[currentHandle as usize].lastTime = CL_ScaledMilliseconds();
    cinTable[currentHandle as usize].startTime = cinTable[currentHandle as usize].lastTime;
    cinTable[currentHandle as usize].RoQPlayed = 24;
    /*	get frame rate */
    cinTable[currentHandle as usize].roqFPS =
        (cin.file[6] as i32 + cin.file[7] as i32 * 256) as isize;
    if cinTable[currentHandle as usize].roqFPS == 0 {
        cinTable[currentHandle as usize].roqFPS = 30
    }
    cinTable[currentHandle as usize].numQuads = -1;
    cinTable[currentHandle as usize].roq_id =
        (cin.file[8] as i32 + cin.file[9] as i32 * 256) as u32;
    cinTable[currentHandle as usize].RoQFrameSize =
        (cin.file[10] as i32 + cin.file[11] as i32 * 256 + cin.file[12] as i32 * 65536) as u32;
    cinTable[currentHandle as usize].roq_flags =
        (cin.file[14] as i32 + cin.file[15] as i32 * 256) as isize;
    if cinTable[currentHandle as usize].RoQFrameSize > 65536
        || cinTable[currentHandle as usize].RoQFrameSize == 0
    {
        return;
    };
}
/* *****************************************************************************
*
* Function:
*
* Description:
*
******************************************************************************/

unsafe extern "C" fn RoQShutdown() {
    let mut s: *const i8 = 0 as *const i8;
    if cinTable[currentHandle as usize].buf.is_null() {
        return;
    }
    if cinTable[currentHandle as usize].status == crate::src::qcommon::q_shared::FMV_IDLE {
        return;
    }
    crate::src::qcommon::common::Com_DPrintf(b"finished cinematic\n\x00" as *const u8 as *const i8);
    cinTable[currentHandle as usize].status = crate::src::qcommon::q_shared::FMV_IDLE;
    if cinTable[currentHandle as usize].iFile != 0 {
        crate::src::qcommon::files::FS_FCloseFile(cinTable[currentHandle as usize].iFile);
        cinTable[currentHandle as usize].iFile = 0
    }
    if cinTable[currentHandle as usize].alterGameState as u64 != 0 {
        crate::src::client::cl_main::clc.state = crate::src::qcommon::q_shared::CA_DISCONNECTED;
        // we can't just do a vstr nextmap, because
        // if we are aborting the intro cinematic with
        // a devmap command, nextmap would be valid by
        // the time it was referenced
        s = crate::src::qcommon::cvar::Cvar_VariableString(
            b"nextmap\x00" as *const u8 as *const i8,
        );
        if *s.offset(0) != 0 {
            crate::src::qcommon::cmd::Cbuf_ExecuteText(
                crate::src::qcommon::q_shared::EXEC_APPEND as i32,
                crate::src::qcommon::q_shared::va(b"%s\n\x00" as *const u8 as *mut i8, s),
            );
            crate::src::qcommon::cvar::Cvar_Set(
                b"nextmap\x00" as *const u8 as *const i8,
                b"\x00" as *const u8 as *const i8,
            );
        }
        CL_handle = -(1)
    }
    cinTable[currentHandle as usize].fileName[0] = 0;
    currentHandle = -(1);
}
/*
==================
CIN_StopCinematic
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CIN_StopCinematic(
    mut handle: i32,
) -> crate::src::qcommon::q_shared::e_status {
    if handle < 0
        || handle >= 16
        || cinTable[handle as usize].status == crate::src::qcommon::q_shared::FMV_EOF
    {
        return crate::src::qcommon::q_shared::FMV_EOF;
    }
    currentHandle = handle;
    crate::src::qcommon::common::Com_DPrintf(
        b"trFMV::stop(), closing %s\n\x00" as *const u8 as *const i8,
        cinTable[currentHandle as usize].fileName.as_mut_ptr(),
    );
    if cinTable[currentHandle as usize].buf.is_null() {
        return crate::src::qcommon::q_shared::FMV_EOF;
    }
    if cinTable[currentHandle as usize].alterGameState as u64 != 0 {
        if crate::src::client::cl_main::clc.state != crate::src::qcommon::q_shared::CA_CINEMATIC {
            return cinTable[currentHandle as usize].status;
        }
    }
    cinTable[currentHandle as usize].status = crate::src::qcommon::q_shared::FMV_EOF;
    RoQShutdown();
    return crate::src::qcommon::q_shared::FMV_EOF;
}
/*
==================
CIN_RunCinematic

Fetch and decompress the pending frame
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CIN_RunCinematic(
    mut handle: i32,
) -> crate::src::qcommon::q_shared::e_status {
    let mut start: i32 = 0;
    let mut thisTime: i32 = 0;
    if handle < 0
        || handle >= 16
        || cinTable[handle as usize].status == crate::src::qcommon::q_shared::FMV_EOF
    {
        return crate::src::qcommon::q_shared::FMV_EOF;
    }
    if cin.currentHandle != handle {
        currentHandle = handle;
        cin.currentHandle = currentHandle;
        cinTable[currentHandle as usize].status = crate::src::qcommon::q_shared::FMV_EOF;
        RoQReset();
    }
    if cinTable[handle as usize].playonwalls < -(1) {
        return cinTable[handle as usize].status;
    }
    currentHandle = handle;
    if cinTable[currentHandle as usize].alterGameState as u64 != 0 {
        if crate::src::client::cl_main::clc.state != crate::src::qcommon::q_shared::CA_CINEMATIC {
            return cinTable[currentHandle as usize].status;
        }
    }
    if cinTable[currentHandle as usize].status == crate::src::qcommon::q_shared::FMV_IDLE {
        return cinTable[currentHandle as usize].status;
    }
    thisTime = CL_ScaledMilliseconds();
    if cinTable[currentHandle as usize].shader != 0
        && crate::stdlib::abs(thisTime - cinTable[currentHandle as usize].lastTime) > 100
    {
        cinTable[currentHandle as usize].startTime +=
            thisTime - cinTable[currentHandle as usize].lastTime
    }
    cinTable[currentHandle as usize].tfps =
        ((CL_ScaledMilliseconds() - cinTable[currentHandle as usize].startTime) * 3 / 100) as isize;
    start = cinTable[currentHandle as usize].startTime;
    while cinTable[currentHandle as usize].tfps != cinTable[currentHandle as usize].numQuads
        && cinTable[currentHandle as usize].status == crate::src::qcommon::q_shared::FMV_PLAY
    {
        RoQInterrupt();
        if start != cinTable[currentHandle as usize].startTime {
            cinTable[currentHandle as usize].tfps =
                ((CL_ScaledMilliseconds() - cinTable[currentHandle as usize].startTime) * 3 / 100)
                    as isize;
            start = cinTable[currentHandle as usize].startTime
        }
    }
    cinTable[currentHandle as usize].lastTime = thisTime;
    if cinTable[currentHandle as usize].status == crate::src::qcommon::q_shared::FMV_LOOPED {
        cinTable[currentHandle as usize].status = crate::src::qcommon::q_shared::FMV_PLAY
    }
    if cinTable[currentHandle as usize].status == crate::src::qcommon::q_shared::FMV_EOF {
        if cinTable[currentHandle as usize].looping as u64 != 0 {
            RoQReset();
        } else {
            RoQShutdown();
            return crate::src::qcommon::q_shared::FMV_EOF;
        }
    }
    return cinTable[currentHandle as usize].status;
}
/*
==================
CIN_PlayCinematic
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CIN_PlayCinematic(
    mut arg: *const i8,
    mut x: i32,
    mut y: i32,
    mut w: i32,
    mut h: i32,
    mut systemBits: i32,
) -> i32 {
    let mut RoQID: u16 = 0;
    let mut name: [i8; 4096] = [0; 4096];
    let mut i: i32 = 0;
    if crate::stdlib::strstr(arg, b"/\x00" as *const u8 as *const i8).is_null()
        && crate::stdlib::strstr(arg, b"\\\x00" as *const u8 as *const i8).is_null()
    {
        crate::src::qcommon::q_shared::Com_sprintf(
            name.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 4096]>() as i32,
            b"video/%s\x00" as *const u8 as *const i8,
            arg,
        );
    } else {
        crate::src::qcommon::q_shared::Com_sprintf(
            name.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 4096]>() as i32,
            b"%s\x00" as *const u8 as *const i8,
            arg,
        );
    }
    if systemBits & 1 == 0 {
        i = 0;
        while i < 16 {
            if crate::stdlib::strcmp(
                cinTable[i as usize].fileName.as_mut_ptr(),
                name.as_mut_ptr(),
            ) == 0
            {
                return i;
            }
            i += 1
        }
    }
    crate::src::qcommon::common::Com_DPrintf(
        b"CIN_PlayCinematic( %s )\n\x00" as *const u8 as *const i8,
        arg,
    );
    crate::stdlib::memset(
        &mut cin as *mut cinematics_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<cinematics_t>(),
    );
    currentHandle = CIN_HandleForVideo();
    cin.currentHandle = currentHandle;
    crate::stdlib::strcpy(
        cinTable[currentHandle as usize].fileName.as_mut_ptr(),
        name.as_mut_ptr(),
    );
    cinTable[currentHandle as usize].ROQSize = 0;
    cinTable[currentHandle as usize].ROQSize = crate::src::qcommon::files::FS_FOpenFileRead(
        cinTable[currentHandle as usize].fileName.as_mut_ptr(),
        &mut (*cinTable.as_mut_ptr().offset(currentHandle as isize)).iFile,
        crate::src::qcommon::q_shared::qtrue,
    );
    if cinTable[currentHandle as usize].ROQSize <= 0 {
        crate::src::qcommon::common::Com_DPrintf(
            b"play(%s), ROQSize<=0\n\x00" as *const u8 as *const i8,
            arg,
        );
        cinTable[currentHandle as usize].fileName[0] = 0;
        return -(1i32);
    }
    CIN_SetExtents(currentHandle, x, y, w, h);
    CIN_SetLooping(
        currentHandle,
        (systemBits & 2 != 0) as crate::src::qcommon::q_shared::qboolean,
    );
    cinTable[currentHandle as usize].CIN_HEIGHT = 512;
    cinTable[currentHandle as usize].CIN_WIDTH = 512;
    cinTable[currentHandle as usize].holdAtEnd =
        (systemBits & 4 != 0) as crate::src::qcommon::q_shared::qboolean;
    cinTable[currentHandle as usize].alterGameState =
        (systemBits & 1 != 0) as crate::src::qcommon::q_shared::qboolean;
    cinTable[currentHandle as usize].playonwalls = 1;
    cinTable[currentHandle as usize].silent =
        (systemBits & 8 != 0) as crate::src::qcommon::q_shared::qboolean;
    cinTable[currentHandle as usize].shader =
        (systemBits & 16 != 0) as crate::src::qcommon::q_shared::qboolean;
    if cinTable[currentHandle as usize].alterGameState as u64 != 0 {
        // close the menu
        if !crate::src::client::cl_ui::uivm.is_null() {
            crate::src::qcommon::vm::VM_Call(
                crate::src::client::cl_ui::uivm,
                crate::ui_public_h::UI_SET_ACTIVE_MENU as i32,
                crate::ui_public_h::UIMENU_NONE as i32,
            );
        }
    } else {
        cinTable[currentHandle as usize].playonwalls =
            (*crate::src::client::cl_main::cl_inGameVideo).integer
    }
    initRoQ();
    crate::src::qcommon::files::FS_Read(
        cin.file.as_mut_ptr() as *mut libc::c_void,
        16,
        cinTable[currentHandle as usize].iFile,
    );
    RoQID = (cin.file[0] as i32 + cin.file[1] as i32 * 256) as u16;
    if RoQID as i32 == 0x1084 {
        RoQ_init();
        //		FS_Read (cin.file, cinTable[currentHandle].RoQFrameSize+8, cinTable[currentHandle].iFile);
        cinTable[currentHandle as usize].status = crate::src::qcommon::q_shared::FMV_PLAY;
        crate::src::qcommon::common::Com_DPrintf(
            b"trFMV::play(), playing %s\n\x00" as *const u8 as *const i8,
            arg,
        );
        if cinTable[currentHandle as usize].alterGameState as u64 != 0 {
            crate::src::client::cl_main::clc.state = crate::src::qcommon::q_shared::CA_CINEMATIC
        }
        crate::src::client::cl_console::Con_Close();
        if cinTable[currentHandle as usize].silent as u64 == 0 {
            crate::src::client::snd_dma::s_rawend[0] = s_soundtime
        }
        return currentHandle;
    }
    crate::src::qcommon::common::Com_DPrintf(
        b"trFMV::play(), invalid RoQ ID\n\x00" as *const u8 as *const i8,
    );
    RoQShutdown();
    return -(1);
}
#[no_mangle]

pub unsafe extern "C" fn CIN_SetExtents(
    mut handle: i32,
    mut x: i32,
    mut y: i32,
    mut w: i32,
    mut h: i32,
) {
    if handle < 0
        || handle >= 16
        || cinTable[handle as usize].status == crate::src::qcommon::q_shared::FMV_EOF
    {
        return;
    }
    cinTable[handle as usize].xpos = x;
    cinTable[handle as usize].ypos = y;
    cinTable[handle as usize].width = w;
    cinTable[handle as usize].height = h;
    cinTable[handle as usize].dirty = crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn CIN_SetLooping(
    mut handle: i32,
    mut loop_0: crate::src::qcommon::q_shared::qboolean,
) {
    if handle < 0
        || handle >= 16
        || cinTable[handle as usize].status == crate::src::qcommon::q_shared::FMV_EOF
    {
        return;
    }
    cinTable[handle as usize].looping = loop_0;
}
/*
==================
CIN_ResampleCinematic

Resample cinematic to 256x256 and store in buf2
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CIN_ResampleCinematic(mut handle: i32, mut buf2: *mut i32) {
    let mut ix: i32 = 0;
    let mut iy: i32 = 0;
    let mut buf3: *mut i32 = 0 as *mut i32;
    let mut xm: i32 = 0;
    let mut ym: i32 = 0;
    let mut ll: i32 = 0;
    let mut buf: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    buf = cinTable[handle as usize].buf;
    xm = cinTable[handle as usize].CIN_WIDTH / 256;
    ym = cinTable[handle as usize].CIN_HEIGHT / 256;
    ll = 8;
    if cinTable[handle as usize].CIN_WIDTH == 512 {
        ll = 9
    }
    buf3 = buf as *mut i32;
    if xm == 2 && ym == 2 {
        let mut bc2: *mut crate::src::qcommon::q_shared::byte =
            0 as *mut crate::src::qcommon::q_shared::byte;
        let mut bc3: *mut crate::src::qcommon::q_shared::byte =
            0 as *mut crate::src::qcommon::q_shared::byte;
        let mut _ic: i32 = 0;
        let mut iiy: i32 = 0;
        bc2 = buf2 as *mut crate::src::qcommon::q_shared::byte;
        bc3 = buf3 as *mut crate::src::qcommon::q_shared::byte;
        iy = 0;
        while iy < 256 {
            iiy = iy << 12;
            ix = 0;
            while ix < 2048 {
                for ic in ix..ix + 4 {
                    *bc2 = (*bc3.offset((iiy + ic) as isize) as i32
                        + *bc3.offset((iiy + 4 + ic) as isize) as i32
                        + *bc3.offset((iiy + 2048 + ic) as isize) as i32
                        + *bc3.offset((iiy + 2048 + 4 + ic) as isize) as i32
                        >> 2) as crate::src::qcommon::q_shared::byte;

                    bc2 = bc2.offset(1);
                }
                ix += 8
            }
            iy += 1
        }
    } else if xm == 2 && ym == 1 {
        let mut bc2_0: *mut crate::src::qcommon::q_shared::byte =
            0 as *mut crate::src::qcommon::q_shared::byte;
        let mut bc3_0: *mut crate::src::qcommon::q_shared::byte =
            0 as *mut crate::src::qcommon::q_shared::byte;
        let mut _ic_0: i32 = 0;
        let mut iiy_0: i32 = 0;
        bc2_0 = buf2 as *mut crate::src::qcommon::q_shared::byte;
        bc3_0 = buf3 as *mut crate::src::qcommon::q_shared::byte;
        iy = 0;
        while iy < 256 {
            iiy_0 = iy << 11;
            ix = 0;
            while ix < 2048 {
                for ic_0 in ix..ix + 4 {
                    *bc2_0 = (*bc3_0.offset((iiy_0 + ic_0) as isize) as i32
                        + *bc3_0.offset((iiy_0 + 4 + ic_0) as isize) as i32
                        >> 1) as crate::src::qcommon::q_shared::byte;

                    bc2_0 = bc2_0.offset(1);
                }
                ix += 8
            }
            iy += 1
        }
    } else {
        iy = 0;
        while iy < 256 {
            ix = 0;
            while ix < 256 {
                *buf2.offset(((iy << 8) + ix) as isize) =
                    *buf3.offset(((iy * ym << ll) + ix * xm) as isize);
                ix += 1
            }
            iy += 1
        }
    };
}
/*
==================
CIN_DrawCinematic
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CIN_DrawCinematic(mut handle: i32) {
    let mut x: f32 = 0.;
    let mut y: f32 = 0.;
    let mut w: f32 = 0.;
    let mut h: f32 = 0.;
    let mut buf: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    if handle < 0
        || handle >= 16
        || cinTable[handle as usize].status == crate::src::qcommon::q_shared::FMV_EOF
    {
        return;
    }
    if cinTable[handle as usize].buf.is_null() {
        return;
    }
    x = cinTable[handle as usize].xpos as f32;
    y = cinTable[handle as usize].ypos as f32;
    w = cinTable[handle as usize].width as f32;
    h = cinTable[handle as usize].height as f32;
    buf = cinTable[handle as usize].buf;
    crate::src::client::cl_scrn::SCR_AdjustFrom640(&mut x, &mut y, &mut w, &mut h);
    if cinTable[handle as usize].dirty != 0
        && (cinTable[handle as usize].CIN_WIDTH as isize != cinTable[handle as usize].drawX
            || cinTable[handle as usize].CIN_HEIGHT as isize != cinTable[handle as usize].drawY)
    {
        let mut buf2: *mut i32 = 0 as *mut i32;
        buf2 = crate::src::qcommon::common::Hunk_AllocateTempMemory(256 * 256 * 4) as *mut i32;
        CIN_ResampleCinematic(handle, buf2);
        crate::src::client::cl_main::re
            .DrawStretchRaw
            .expect("non-null function pointer")(
            x as i32,
            y as i32,
            w as i32,
            h as i32,
            256,
            256,
            buf2 as *mut crate::src::qcommon::q_shared::byte,
            handle,
            crate::src::qcommon::q_shared::qtrue,
        );
        cinTable[handle as usize].dirty = crate::src::qcommon::q_shared::qfalse;
        crate::src::qcommon::common::Hunk_FreeTempMemory(buf2 as *mut libc::c_void);
        return;
    }
    crate::src::client::cl_main::re
        .DrawStretchRaw
        .expect("non-null function pointer")(
        x as i32,
        y as i32,
        w as i32,
        h as i32,
        cinTable[handle as usize].drawX as i32,
        cinTable[handle as usize].drawY as i32,
        buf,
        handle,
        cinTable[handle as usize].dirty,
    );
    cinTable[handle as usize].dirty = crate::src::qcommon::q_shared::qfalse;
}
#[no_mangle]

pub unsafe extern "C" fn CL_PlayCinematic_f() {
    let mut arg: *mut i8 = 0 as *mut i8;
    let mut s: *mut i8 = 0 as *mut i8;
    let mut bits: i32 = 1;
    crate::src::qcommon::common::Com_DPrintf(b"CL_PlayCinematic_f\n\x00" as *const u8 as *const i8);
    if crate::src::client::cl_main::clc.state == crate::src::qcommon::q_shared::CA_CINEMATIC {
        SCR_StopCinematic();
    }
    arg = crate::src::qcommon::cmd::Cmd_Argv(1);
    s = crate::src::qcommon::cmd::Cmd_Argv(2);
    if !s.is_null() && *s.offset(0) as i32 == '1' as i32
        || crate::src::qcommon::q_shared::Q_stricmp(
            arg,
            b"demoend.roq\x00" as *const u8 as *const i8,
        ) == 0
        || crate::src::qcommon::q_shared::Q_stricmp(arg, b"end.roq\x00" as *const u8 as *const i8)
            == 0
    {
        bits |= 4
    }
    if !s.is_null() && *s.offset(0) as i32 == '2' as i32 {
        bits |= 2
    }
    crate::src::client::snd_main::S_StopAllSounds();
    CL_handle = CIN_PlayCinematic(arg, 0, 0, 640, 480, bits);
    if CL_handle >= 0 {
        loop {
            SCR_RunCinematic();
            if !(cinTable[currentHandle as usize].buf.is_null()
                && cinTable[currentHandle as usize].status
                    == crate::src::qcommon::q_shared::FMV_PLAY)
            {
                break;
            }
        }
        // wait for first frame (load codebook and sound)
    };
}
#[no_mangle]

pub unsafe extern "C" fn SCR_DrawCinematic() {
    if CL_handle >= 0 && CL_handle < 16 {
        CIN_DrawCinematic(CL_handle);
    };
}
#[no_mangle]

pub unsafe extern "C" fn SCR_RunCinematic() {
    if CL_handle >= 0 && CL_handle < 16 {
        CIN_RunCinematic(CL_handle);
    };
}
#[no_mangle]

pub unsafe extern "C" fn SCR_StopCinematic() {
    if CL_handle >= 0 && CL_handle < 16 {
        CIN_StopCinematic(CL_handle);
        crate::src::client::snd_main::S_StopAllSounds();
        CL_handle = -(1)
    };
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
// client.h -- primary header for client
/* USE_CURL */
// file full of random crap that gets used to create cl_guid
// time between connection packet retransmits
// snapshots are a view of the server at a given time
// cleared if delta parsing was invalid
// rate delayed and dropped commands
// server time the message is valid for (in msec)
// copied from netchan->incoming_sequence
// messageNum the delta is from
// time from when cmdNum-1 was sent to time packet was reeceived
// portalarea visibility bits
// the next cmdNum the server is expecting
// complete information about the current player at this time
// all of the entities that need to be presented
// at the time of this snapshot
// execute all commands up to this before
// making the snapshot current
/*
=============================================================================

the clientActive_t structure is wiped completely at every
new gamestate_t, potentially several times during an established connection

=============================================================================
*/
// cl.cmdNumber when packet was sent
// usercmd->serverTime when packet was sent
// cls.realtime when packet was sent
// the parseEntities array must be large enough to hold PACKET_BACKUP frames of
// entities, so that when a delta compressed message arives from the server
// it can be un-deltad from the original
// it requres several frames in a timeout condition
// to disconnect, preventing debugging breaks from
// causing immediate disconnects on continue
// latest received from server
// may be paused during play
// to prevent time from flowing bakcwards
// to check tournament restarts
// cl.serverTime = cls.realtime + cl.serverTimeDelta
// this value changes as net lag varies
// set if any cgame frame has been forced to extrapolate
// cleared when CL_AdjustTimeDelta looks at it
// set on parse of any valid packet
// configstrings
// extracted from CS_SERVERINFO
// index (not anded off) into cl_parse_entities[]
// added to by mouse events
// set by joystick events
// cgame communicates a few values to the client system
// current weapon to add to usercmd_t
// cmds[cmdNumber] is the predicted command, [cmdNumber-1] is the last
// properly generated command
// each mesage will send several old cmds
// incremented each frame, because multiple
// frames may need to be packed into a single packet
// information about each packet we have sent out
// the client maintains its own idea of view angles, which are
// sent to the server each frame.  It is cleared to 0 upon entering each level.
// the server sends a delta each frame which is added to the locally
// tracked view angles to account for standing on rotating objects,
// and teleport direction changes
// included in each client message so the server
// can tell if it is for a prior map_restart
// big stuff at end of structure so most offsets are 15 bits or less
// for delta compression when not in previous frame
/*
=============================================================================

the clientConnection_t structure is wiped when disconnecting from a server,
either to go to a full screen console, play a demo, or connect to a different server

A connection can be to either a server through the network layer or a
demo through a file.

=============================================================================
*/
// connection status
// for retransmits during connection
// for timeouts
// name of server from original connect (used by reconnect)
// for connection retransmits
// for display on connection dialog
// for display on connection dialog
// from the server to use for connecting
// from the server for checksum calculations
// these are our reliable messages that go to the server
// the last one the server has executed
// server message (unreliable) and command (reliable) sequence
// numbers are NOT cleared at level changes, but continue to
// increase as long as the connection is valid
// message sequence is used by both the network layer and the
// delta compression layer
// reliable messages received from server
// last server command grabbed or executed with CL_GetServerCommand
// file transfer from server
/* USE_CURL */
// block we are waiting for
// how many bytes we got
// how many bytes we got
// list of paks we need to download
// if true, we need to do another FS_Restart because we downloaded a pak
// demo information
// don't record until a non-delta message is received
// counter of rendered frames
// cls.realtime before first frame
// each frame will be at this time + frameNum * 50
// time the last frame was rendered
// minimum frame duration
// maximum frame duration
// log of frame durations
// incoming data...
// !!! FIXME: convert from parallel arrays to array of a struct.
// outgoing data...
// if voipTargets[i / 8] & (1 << (i % 8)),
// then we are sending to clientnum i.
// big stuff at end of structure so most offsets are 15 bits or less
/*
==================================================================

the clientStatic_t structure is never wiped, and is used even when
no client connection is active at all
(except when CL_Shutdown is called)

==================================================================
*/
// bring up the cd needed dialog next frame
// when the server clears the hunk, all of these must be restarted
// msec since last frame
// ignores pause
// ignoring pause, so console always works
// additional global servers
// source currently pinging or updating
// update server info
// rendering info
//=============================================================================
// interface to cgame dll or vm
// interface to ui dll or vm
// interface to refresh .dll
//
// cvars
//
// cl_voipSendTarget is a string: "all" to broadcast to everyone, "none" to
//  send to no one, or a comma-separated list of client numbers:
//  "0,7,2,23" ... an empty string is treated like "all".
// 20ms at 48k
// 3 frame is 60ms of audio, the max opus will encode at once
//=================================================
//
// cl_main
//
//
// cl_input
//
// key nums holding it down
// msec timestamp
// msec down this frame if both a down and up happened
// current state
// set when down, not cleared when up
//
// cl_parse.c
//
//====================================================================
//
// console
//
//
// cl_scrn.c
//
// returns in virtual 640x480 coordinates
// draws a string with embedded color control characters with fade
// ignores embedded color control characters
//
// cl_cin.c
//
#[no_mangle]

pub unsafe extern "C" fn CIN_UploadCinematic(mut handle: i32) {
    if handle >= 0 && handle < 16 {
        if cinTable[handle as usize].buf.is_null() {
            return;
        }
        if cinTable[handle as usize].playonwalls <= 0 && cinTable[handle as usize].dirty != 0 {
            if cinTable[handle as usize].playonwalls == 0 {
                cinTable[handle as usize].playonwalls = -(1)
            } else if cinTable[handle as usize].playonwalls == -(1) {
                cinTable[handle as usize].playonwalls = -(2)
            } else {
                cinTable[handle as usize].dirty = crate::src::qcommon::q_shared::qfalse
            }
        }
        // Resample the video if needed
        if cinTable[handle as usize].dirty != 0
            && (cinTable[handle as usize].CIN_WIDTH as isize != cinTable[handle as usize].drawX
                || cinTable[handle as usize].CIN_HEIGHT as isize != cinTable[handle as usize].drawY)
        {
            let mut buf2: *mut i32 = 0 as *mut i32;
            buf2 = crate::src::qcommon::common::Hunk_AllocateTempMemory(256 * 256 * 4) as *mut i32;
            CIN_ResampleCinematic(handle, buf2);
            crate::src::client::cl_main::re
                .UploadCinematic
                .expect("non-null function pointer")(
                cinTable[handle as usize].CIN_WIDTH,
                cinTable[handle as usize].CIN_HEIGHT,
                256,
                256,
                buf2 as *mut crate::src::qcommon::q_shared::byte,
                handle,
                crate::src::qcommon::q_shared::qtrue,
            );
            cinTable[handle as usize].dirty = crate::src::qcommon::q_shared::qfalse;
            crate::src::qcommon::common::Hunk_FreeTempMemory(buf2 as *mut libc::c_void);
        } else {
            // Upload video at normal resolution
            crate::src::client::cl_main::re
                .UploadCinematic
                .expect("non-null function pointer")(
                cinTable[handle as usize].CIN_WIDTH,
                cinTable[handle as usize].CIN_HEIGHT,
                cinTable[handle as usize].drawX as i32,
                cinTable[handle as usize].drawY as i32,
                cinTable[handle as usize].buf,
                handle,
                cinTable[handle as usize].dirty,
            );
            cinTable[handle as usize].dirty = crate::src::qcommon::q_shared::qfalse
        }
        if (*crate::src::client::cl_main::cl_inGameVideo).integer == 0
            && cinTable[handle as usize].playonwalls == 1
        {
            cinTable[handle as usize].playonwalls -= 1
        } else if (*crate::src::client::cl_main::cl_inGameVideo).integer != 0
            && cinTable[handle as usize].playonwalls != 1
        {
            cinTable[handle as usize].playonwalls = 1
        }
    };
}
