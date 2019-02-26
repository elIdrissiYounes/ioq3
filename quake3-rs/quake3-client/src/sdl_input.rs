#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, custom_attribute, extern_types, libc)]
extern crate libc;
#[header_src = "/usr/include/bits/types.h"]
pub mod types_h {
    pub type __uint8_t = libc::c_uchar;
    pub type __int16_t = libc::c_short;
    pub type __uint16_t = libc::c_ushort;
    pub type __int32_t = libc::c_int;
    pub type __uint32_t = libc::c_uint;
    pub type __int64_t = libc::c_long;
    use super::{libc};
}
#[header_src = "/usr/include/bits/stdint-intn.h"]
pub mod stdint_intn_h {
    pub type int16_t = __int16_t;
    pub type int32_t = __int32_t;
    pub type int64_t = __int64_t;
    use super::types_h::{__int16_t, __int32_t, __int64_t};
}
#[header_src = "/usr/include/bits/stdint-uintn.h"]
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    pub type uint16_t = __uint16_t;
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[header_src = "/usr/include/SDL2/SDL_stdinc.h"]
pub mod SDL_stdinc_h {
    /*
  Simple DirectMedia Layer
  Copyright (C) 1997-2018 Sam Lantinga <slouken@libsdl.org>

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.
*/
    /* *
 *  \file SDL_stdinc.h
 *
 *  This is a general header that includes C language support.
 */
    /* *
 *  The number of elements in an array.
 */
    /* *
 *  Macro useful for building other macros with strings in them
 *
 *  e.g. #define LOG_ERROR(X) OutputDebugString(SDL_STRINGIFY_ARG(__FUNCTION__) ": " X "\n")
 */
    /* *
 *  \name Cast operators
 *
 *  Use proper C++ casts when compiled as C++ to be compatible with the option
 *  -Wold-style-cast of GCC (and -Werror=old-style-cast in GCC 4.2 and above).
 */
/* @{ */
    /* @} */
    /* Cast operators */
    /* Define a four character code as a Uint32 */
    /* *
 *  \name Basic data types
 */
/* @{ */
    pub type SDL_bool = libc::c_uint;
    pub const SDL_TRUE: SDL_bool = 1;
    pub const SDL_FALSE: SDL_bool = 0;
    /* *
 * \brief An unsigned 8-bit integer type.
 */
    /* 255 */
    /* 0 */
    pub type Uint8 = uint8_t;
    /* *
 * \brief A signed 16-bit integer type.
 */
    /* 32767 */
    /* -32768 */
    pub type Sint16 = int16_t;
    /* *
 * \brief An unsigned 16-bit integer type.
 */
    /* 65535 */
    /* 0 */
    pub type Uint16 = uint16_t;
    /* *
 * \brief A signed 32-bit integer type.
 */
    /* 2147483647 */
    /* -2147483648 */
    pub type Sint32 = int32_t;
    /* *
 * \brief An unsigned 32-bit integer type.
 */
    /* 4294967295 */
    /* 0 */
    pub type Uint32 = uint32_t;
    /* *
 * \brief A signed 64-bit integer type.
 */
    /* 9223372036854775807 */
    /* -9223372036854775808 */
    pub type Sint64 = int64_t;
    use super::{libc};
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t, int64_t};
}
#[header_src = "/usr/include/SDL2/SDL_video.h"]
pub mod SDL_video_h {
    /* *
 *  \brief The flags on a window
 *
 *  \sa SDL_GetWindowFlags()
 */
    pub type unnamed = libc::c_uint;
    /* *< window usable for Vulkan surface */
    pub const SDL_WINDOW_VULKAN: unnamed = 268435456;
    /* *< window should be treated as a popup menu */
    pub const SDL_WINDOW_POPUP_MENU: unnamed = 524288;
    /* *< window should be treated as a tooltip */
    pub const SDL_WINDOW_TOOLTIP: unnamed = 262144;
    /* *< window should be treated as a utility window */
    pub const SDL_WINDOW_UTILITY: unnamed = 131072;
    /* *< window should not be added to the taskbar */
    pub const SDL_WINDOW_SKIP_TASKBAR: unnamed = 65536;
    /* *< window should always be above others */
    pub const SDL_WINDOW_ALWAYS_ON_TOP: unnamed = 32768;
    /* *< window has mouse captured (unrelated to INPUT_GRABBED) */
    pub const SDL_WINDOW_MOUSE_CAPTURE: unnamed = 16384;
    /* *< window should be created in high-DPI mode if supported.
                                                     On macOS NSHighResolutionCapable must be set true in the
                                                     application's Info.plist for this to have any effect. */
    pub const SDL_WINDOW_ALLOW_HIGHDPI: unnamed = 8192;
    /* *< window not created by SDL */
    pub const SDL_WINDOW_FOREIGN: unnamed = 2048;
    pub const SDL_WINDOW_FULLSCREEN_DESKTOP: unnamed = 4097;
    /* *< window has mouse focus */
    pub const SDL_WINDOW_MOUSE_FOCUS: unnamed = 1024;
    /* *< window has input focus */
    pub const SDL_WINDOW_INPUT_FOCUS: unnamed = 512;
    /* *< window has grabbed input focus */
    pub const SDL_WINDOW_INPUT_GRABBED: unnamed = 256;
    /* *< window is maximized */
    pub const SDL_WINDOW_MAXIMIZED: unnamed = 128;
    /* *< window is minimized */
    pub const SDL_WINDOW_MINIMIZED: unnamed = 64;
    /* *< window can be resized */
    pub const SDL_WINDOW_RESIZABLE: unnamed = 32;
    /* *< no window decoration */
    pub const SDL_WINDOW_BORDERLESS: unnamed = 16;
    /* *< window is not visible */
    pub const SDL_WINDOW_HIDDEN: unnamed = 8;
    /* *< window is visible */
    pub const SDL_WINDOW_SHOWN: unnamed = 4;
    /* *< window usable with OpenGL context */
    pub const SDL_WINDOW_OPENGL: unnamed = 2;
    /* !!! FIXME: change this to name = (1<<x). */
    /* *< fullscreen window */
    pub const SDL_WINDOW_FULLSCREEN: unnamed = 1;
    /* *
 *  \brief Used to indicate that you don't care what the window position is.
 */
    /* *
 *  \brief Used to indicate that the window position should be centered.
 */
    /* *
 *  \brief Event subtype for window events
 */
    pub type unnamed_0 = libc::c_uint;
    /* *< Window had a hit test that wasn't SDL_HITTEST_NORMAL. */
    pub const SDL_WINDOWEVENT_HIT_TEST: unnamed_0 = 16;
    /* *< Window is being offered a focus (should SetWindowInputFocus() on itself or a subwindow, or ignore) */
    pub const SDL_WINDOWEVENT_TAKE_FOCUS: unnamed_0 = 15;
    /* *< The window manager requests that the window be closed */
    pub const SDL_WINDOWEVENT_CLOSE: unnamed_0 = 14;
    /* *< Window has lost keyboard focus */
    pub const SDL_WINDOWEVENT_FOCUS_LOST: unnamed_0 = 13;
    /* *< Window has gained keyboard focus */
    pub const SDL_WINDOWEVENT_FOCUS_GAINED: unnamed_0 = 12;
    /* *< Window has lost mouse focus */
    pub const SDL_WINDOWEVENT_LEAVE: unnamed_0 = 11;
    /* *< Window has gained mouse focus */
    pub const SDL_WINDOWEVENT_ENTER: unnamed_0 = 10;
    /* *< Window has been restored to normal size
                                         and position */
    pub const SDL_WINDOWEVENT_RESTORED: unnamed_0 = 9;
    /* *< Window has been maximized */
    pub const SDL_WINDOWEVENT_MAXIMIZED: unnamed_0 = 8;
    /* *< Window has been minimized */
    pub const SDL_WINDOWEVENT_MINIMIZED: unnamed_0 = 7;
    /* *< The window size has changed, either as
                                         a result of an API call or through the
                                         system or user changing the window size. */
    pub const SDL_WINDOWEVENT_SIZE_CHANGED: unnamed_0 = 6;
    /* *< Window has been resized to data1xdata2 */
    pub const SDL_WINDOWEVENT_RESIZED: unnamed_0 = 5;
    /* *< Window has been moved to data1, data2
                                     */
    pub const SDL_WINDOWEVENT_MOVED: unnamed_0 = 4;
    /* *< Window has been exposed and should be
                                         redrawn */
    pub const SDL_WINDOWEVENT_EXPOSED: unnamed_0 = 3;
    /* *< Window has been hidden */
    pub const SDL_WINDOWEVENT_HIDDEN: unnamed_0 = 2;
    /* *< Window has been shown */
    pub const SDL_WINDOWEVENT_SHOWN: unnamed_0 = 1;
    /* *< Never used */
    pub const SDL_WINDOWEVENT_NONE: unnamed_0 = 0;
    use super::{libc};
    use super::SDL_stdinc_h::{Uint32, SDL_bool};
    extern "C" {
        /* *
 *  \brief The type used to identify a window
 *
 *  \sa SDL_CreateWindow()
 *  \sa SDL_CreateWindowFrom()
 *  \sa SDL_DestroyWindow()
 *  \sa SDL_GetWindowData()
 *  \sa SDL_GetWindowFlags()
 *  \sa SDL_GetWindowGrab()
 *  \sa SDL_GetWindowPosition()
 *  \sa SDL_GetWindowSize()
 *  \sa SDL_GetWindowTitle()
 *  \sa SDL_HideWindow()
 *  \sa SDL_MaximizeWindow()
 *  \sa SDL_MinimizeWindow()
 *  \sa SDL_RaiseWindow()
 *  \sa SDL_RestoreWindow()
 *  \sa SDL_SetWindowData()
 *  \sa SDL_SetWindowFullscreen()
 *  \sa SDL_SetWindowGrab()
 *  \sa SDL_SetWindowIcon()
 *  \sa SDL_SetWindowPosition()
 *  \sa SDL_SetWindowSize()
 *  \sa SDL_SetWindowBordered()
 *  \sa SDL_SetWindowResizable()
 *  \sa SDL_SetWindowTitle()
 *  \sa SDL_ShowWindow()
 */
        pub type SDL_Window;
        /* *
 *  \brief Get the window flags.
 */
        #[no_mangle]
        pub fn SDL_GetWindowFlags(window: *mut SDL_Window) -> Uint32;
        /* *
 *  \brief Set a window's input grab mode.
 *
 *  \param window The window for which the input grab mode should be set.
 *  \param grabbed This is SDL_TRUE to grab input, and SDL_FALSE to release input.
 *
 *  If the caller enables a grab while another window is currently grabbed,
 *  the other window loses its grab in favor of the caller's window.
 *
 *  \sa SDL_GetWindowGrab()
 */
        #[no_mangle]
        pub fn SDL_SetWindowGrab(window: *mut SDL_Window, grabbed: SDL_bool);
    }
}
#[header_src = "/usr/include/SDL2/SDL_scancode.h"]
pub mod SDL_scancode_h {
    /*
  Simple DirectMedia Layer
  Copyright (C) 1997-2018 Sam Lantinga <slouken@libsdl.org>

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.
*/
    /* *
 *  \file SDL_scancode.h
 *
 *  Defines keyboard scancodes.
 */
    /* *
 *  \brief The SDL keyboard scancode representation.
 *
 *  Values of this type are used to represent keyboard keys, among other places
 *  in the \link SDL_Keysym::scancode key.keysym.scancode \endlink field of the
 *  SDL_Event structure.
 *
 *  The values in this enumeration are based on the USB usage page standard:
 *  http://www.usb.org/developers/hidpage/Hut1_12v2.pdf
 */
    pub type SDL_Scancode = libc::c_uint;
    /* @} */
    /* Usage page 0x0C (additional media keys) */
    /* Add any other keys here. */
    /* *< not a key, just marks the number of scancodes
                                 for array bounds */
    pub const SDL_NUM_SCANCODES: SDL_Scancode = 512;
    pub const SDL_SCANCODE_AUDIOFASTFORWARD: SDL_Scancode = 286;
    /* @} */
    /* Walther keys */
    /* *
     *  \name Usage page 0x0C (additional media keys)
     *
     *  These values are mapped from usage page 0x0C (USB consumer page).
     */
    /* @{ */
    pub const SDL_SCANCODE_AUDIOREWIND: SDL_Scancode = 285;
    pub const SDL_SCANCODE_APP2: SDL_Scancode = 284;
    pub const SDL_SCANCODE_APP1: SDL_Scancode = 283;
    pub const SDL_SCANCODE_SLEEP: SDL_Scancode = 282;
    pub const SDL_SCANCODE_EJECT: SDL_Scancode = 281;
    pub const SDL_SCANCODE_KBDILLUMUP: SDL_Scancode = 280;
    pub const SDL_SCANCODE_KBDILLUMDOWN: SDL_Scancode = 279;
    pub const SDL_SCANCODE_KBDILLUMTOGGLE: SDL_Scancode = 278;
    /* *< display mirroring/dual display
                                           switch, video mode switch */
    pub const SDL_SCANCODE_DISPLAYSWITCH: SDL_Scancode = 277;
    pub const SDL_SCANCODE_BRIGHTNESSUP: SDL_Scancode = 276;
    /* @} */
    /* Usage page 0x0C */
    /* *
     *  \name Walther keys
     *
     *  These are values that Christian Walther added (for mac keyboard?).
     */
    /* @{ */
    pub const SDL_SCANCODE_BRIGHTNESSDOWN: SDL_Scancode = 275;
    pub const SDL_SCANCODE_AC_BOOKMARKS: SDL_Scancode = 274;
    pub const SDL_SCANCODE_AC_REFRESH: SDL_Scancode = 273;
    pub const SDL_SCANCODE_AC_STOP: SDL_Scancode = 272;
    pub const SDL_SCANCODE_AC_FORWARD: SDL_Scancode = 271;
    pub const SDL_SCANCODE_AC_BACK: SDL_Scancode = 270;
    pub const SDL_SCANCODE_AC_HOME: SDL_Scancode = 269;
    pub const SDL_SCANCODE_AC_SEARCH: SDL_Scancode = 268;
    pub const SDL_SCANCODE_COMPUTER: SDL_Scancode = 267;
    pub const SDL_SCANCODE_CALCULATOR: SDL_Scancode = 266;
    pub const SDL_SCANCODE_MAIL: SDL_Scancode = 265;
    pub const SDL_SCANCODE_WWW: SDL_Scancode = 264;
    pub const SDL_SCANCODE_MEDIASELECT: SDL_Scancode = 263;
    pub const SDL_SCANCODE_AUDIOMUTE: SDL_Scancode = 262;
    pub const SDL_SCANCODE_AUDIOPLAY: SDL_Scancode = 261;
    pub const SDL_SCANCODE_AUDIOSTOP: SDL_Scancode = 260;
    pub const SDL_SCANCODE_AUDIOPREV: SDL_Scancode = 259;
    /* @} */
    /* Usage page 0x07 */
    /* *
     *  \name Usage page 0x0C
     *
     *  These values are mapped from usage page 0x0C (USB consumer page).
     */
    /* @{ */
    pub const SDL_SCANCODE_AUDIONEXT: SDL_Scancode = 258;
    /* *< I'm not sure if this is really not covered
                                 *   by any of the above, but since there's a
                                 *   special KMOD_MODE for it I'm adding it here
                                 */
    pub const SDL_SCANCODE_MODE: SDL_Scancode = 257;
    /* *< windows, command (apple), meta */
    pub const SDL_SCANCODE_RGUI: SDL_Scancode = 231;
    /* *< alt gr, option */
    pub const SDL_SCANCODE_RALT: SDL_Scancode = 230;
    pub const SDL_SCANCODE_RSHIFT: SDL_Scancode = 229;
    pub const SDL_SCANCODE_RCTRL: SDL_Scancode = 228;
    /* *< windows, command (apple), meta */
    pub const SDL_SCANCODE_LGUI: SDL_Scancode = 227;
    /* *< alt, option */
    pub const SDL_SCANCODE_LALT: SDL_Scancode = 226;
    pub const SDL_SCANCODE_LSHIFT: SDL_Scancode = 225;
    pub const SDL_SCANCODE_LCTRL: SDL_Scancode = 224;
    pub const SDL_SCANCODE_KP_HEXADECIMAL: SDL_Scancode = 221;
    pub const SDL_SCANCODE_KP_DECIMAL: SDL_Scancode = 220;
    pub const SDL_SCANCODE_KP_OCTAL: SDL_Scancode = 219;
    pub const SDL_SCANCODE_KP_BINARY: SDL_Scancode = 218;
    pub const SDL_SCANCODE_KP_CLEARENTRY: SDL_Scancode = 217;
    pub const SDL_SCANCODE_KP_CLEAR: SDL_Scancode = 216;
    pub const SDL_SCANCODE_KP_PLUSMINUS: SDL_Scancode = 215;
    pub const SDL_SCANCODE_KP_MEMDIVIDE: SDL_Scancode = 214;
    pub const SDL_SCANCODE_KP_MEMMULTIPLY: SDL_Scancode = 213;
    pub const SDL_SCANCODE_KP_MEMSUBTRACT: SDL_Scancode = 212;
    pub const SDL_SCANCODE_KP_MEMADD: SDL_Scancode = 211;
    pub const SDL_SCANCODE_KP_MEMCLEAR: SDL_Scancode = 210;
    pub const SDL_SCANCODE_KP_MEMRECALL: SDL_Scancode = 209;
    pub const SDL_SCANCODE_KP_MEMSTORE: SDL_Scancode = 208;
    pub const SDL_SCANCODE_KP_EXCLAM: SDL_Scancode = 207;
    pub const SDL_SCANCODE_KP_AT: SDL_Scancode = 206;
    pub const SDL_SCANCODE_KP_SPACE: SDL_Scancode = 205;
    pub const SDL_SCANCODE_KP_HASH: SDL_Scancode = 204;
    pub const SDL_SCANCODE_KP_COLON: SDL_Scancode = 203;
    pub const SDL_SCANCODE_KP_DBLVERTICALBAR: SDL_Scancode = 202;
    pub const SDL_SCANCODE_KP_VERTICALBAR: SDL_Scancode = 201;
    pub const SDL_SCANCODE_KP_DBLAMPERSAND: SDL_Scancode = 200;
    pub const SDL_SCANCODE_KP_AMPERSAND: SDL_Scancode = 199;
    pub const SDL_SCANCODE_KP_GREATER: SDL_Scancode = 198;
    pub const SDL_SCANCODE_KP_LESS: SDL_Scancode = 197;
    pub const SDL_SCANCODE_KP_PERCENT: SDL_Scancode = 196;
    pub const SDL_SCANCODE_KP_POWER: SDL_Scancode = 195;
    pub const SDL_SCANCODE_KP_XOR: SDL_Scancode = 194;
    pub const SDL_SCANCODE_KP_F: SDL_Scancode = 193;
    pub const SDL_SCANCODE_KP_E: SDL_Scancode = 192;
    pub const SDL_SCANCODE_KP_D: SDL_Scancode = 191;
    pub const SDL_SCANCODE_KP_C: SDL_Scancode = 190;
    pub const SDL_SCANCODE_KP_B: SDL_Scancode = 189;
    pub const SDL_SCANCODE_KP_A: SDL_Scancode = 188;
    pub const SDL_SCANCODE_KP_BACKSPACE: SDL_Scancode = 187;
    pub const SDL_SCANCODE_KP_TAB: SDL_Scancode = 186;
    pub const SDL_SCANCODE_KP_RIGHTBRACE: SDL_Scancode = 185;
    pub const SDL_SCANCODE_KP_LEFTBRACE: SDL_Scancode = 184;
    pub const SDL_SCANCODE_KP_RIGHTPAREN: SDL_Scancode = 183;
    pub const SDL_SCANCODE_KP_LEFTPAREN: SDL_Scancode = 182;
    pub const SDL_SCANCODE_CURRENCYSUBUNIT: SDL_Scancode = 181;
    pub const SDL_SCANCODE_CURRENCYUNIT: SDL_Scancode = 180;
    pub const SDL_SCANCODE_DECIMALSEPARATOR: SDL_Scancode = 179;
    pub const SDL_SCANCODE_THOUSANDSSEPARATOR: SDL_Scancode = 178;
    pub const SDL_SCANCODE_KP_000: SDL_Scancode = 177;
    pub const SDL_SCANCODE_KP_00: SDL_Scancode = 176;
    pub const SDL_SCANCODE_EXSEL: SDL_Scancode = 164;
    pub const SDL_SCANCODE_CRSEL: SDL_Scancode = 163;
    pub const SDL_SCANCODE_CLEARAGAIN: SDL_Scancode = 162;
    pub const SDL_SCANCODE_OPER: SDL_Scancode = 161;
    pub const SDL_SCANCODE_OUT: SDL_Scancode = 160;
    pub const SDL_SCANCODE_SEPARATOR: SDL_Scancode = 159;
    pub const SDL_SCANCODE_RETURN2: SDL_Scancode = 158;
    pub const SDL_SCANCODE_PRIOR: SDL_Scancode = 157;
    pub const SDL_SCANCODE_CLEAR: SDL_Scancode = 156;
    pub const SDL_SCANCODE_CANCEL: SDL_Scancode = 155;
    pub const SDL_SCANCODE_SYSREQ: SDL_Scancode = 154;
    /* *< Erase-Eaze */
    pub const SDL_SCANCODE_ALTERASE: SDL_Scancode = 153;
    /* *< reserved */
    pub const SDL_SCANCODE_LANG9: SDL_Scancode = 152;
    /* *< reserved */
    pub const SDL_SCANCODE_LANG8: SDL_Scancode = 151;
    /* *< reserved */
    pub const SDL_SCANCODE_LANG7: SDL_Scancode = 150;
    /* *< reserved */
    pub const SDL_SCANCODE_LANG6: SDL_Scancode = 149;
    /* *< Zenkaku/Hankaku */
    pub const SDL_SCANCODE_LANG5: SDL_Scancode = 148;
    /* *< Hiragana */
    pub const SDL_SCANCODE_LANG4: SDL_Scancode = 147;
    /* *< Katakana */
    pub const SDL_SCANCODE_LANG3: SDL_Scancode = 146;
    /* *< Hanja conversion */
    pub const SDL_SCANCODE_LANG2: SDL_Scancode = 145;
    /* *< Hangul/English toggle */
    pub const SDL_SCANCODE_LANG1: SDL_Scancode = 144;
    pub const SDL_SCANCODE_INTERNATIONAL9: SDL_Scancode = 143;
    pub const SDL_SCANCODE_INTERNATIONAL8: SDL_Scancode = 142;
    pub const SDL_SCANCODE_INTERNATIONAL7: SDL_Scancode = 141;
    pub const SDL_SCANCODE_INTERNATIONAL6: SDL_Scancode = 140;
    pub const SDL_SCANCODE_INTERNATIONAL5: SDL_Scancode = 139;
    pub const SDL_SCANCODE_INTERNATIONAL4: SDL_Scancode = 138;
    /* *< Yen */
    pub const SDL_SCANCODE_INTERNATIONAL3: SDL_Scancode = 137;
    pub const SDL_SCANCODE_INTERNATIONAL2: SDL_Scancode = 136;
    /* *< used on Asian keyboards, see
                                            footnotes in USB doc */
    pub const SDL_SCANCODE_INTERNATIONAL1: SDL_Scancode = 135;
    pub const SDL_SCANCODE_KP_EQUALSAS400: SDL_Scancode = 134;
    /* not sure whether there's a reason to enable these */
/*     SDL_SCANCODE_LOCKINGCAPSLOCK = 130,  */
/*     SDL_SCANCODE_LOCKINGNUMLOCK = 131, */
/*     SDL_SCANCODE_LOCKINGSCROLLLOCK = 132, */
    pub const SDL_SCANCODE_KP_COMMA: SDL_Scancode = 133;
    pub const SDL_SCANCODE_VOLUMEDOWN: SDL_Scancode = 129;
    pub const SDL_SCANCODE_VOLUMEUP: SDL_Scancode = 128;
    pub const SDL_SCANCODE_MUTE: SDL_Scancode = 127;
    pub const SDL_SCANCODE_FIND: SDL_Scancode = 126;
    pub const SDL_SCANCODE_PASTE: SDL_Scancode = 125;
    pub const SDL_SCANCODE_COPY: SDL_Scancode = 124;
    pub const SDL_SCANCODE_CUT: SDL_Scancode = 123;
    pub const SDL_SCANCODE_UNDO: SDL_Scancode = 122;
    /* *< redo */
    pub const SDL_SCANCODE_AGAIN: SDL_Scancode = 121;
    pub const SDL_SCANCODE_STOP: SDL_Scancode = 120;
    pub const SDL_SCANCODE_SELECT: SDL_Scancode = 119;
    pub const SDL_SCANCODE_MENU: SDL_Scancode = 118;
    pub const SDL_SCANCODE_HELP: SDL_Scancode = 117;
    pub const SDL_SCANCODE_EXECUTE: SDL_Scancode = 116;
    pub const SDL_SCANCODE_F24: SDL_Scancode = 115;
    pub const SDL_SCANCODE_F23: SDL_Scancode = 114;
    pub const SDL_SCANCODE_F22: SDL_Scancode = 113;
    pub const SDL_SCANCODE_F21: SDL_Scancode = 112;
    pub const SDL_SCANCODE_F20: SDL_Scancode = 111;
    pub const SDL_SCANCODE_F19: SDL_Scancode = 110;
    pub const SDL_SCANCODE_F18: SDL_Scancode = 109;
    pub const SDL_SCANCODE_F17: SDL_Scancode = 108;
    pub const SDL_SCANCODE_F16: SDL_Scancode = 107;
    pub const SDL_SCANCODE_F15: SDL_Scancode = 106;
    pub const SDL_SCANCODE_F14: SDL_Scancode = 105;
    pub const SDL_SCANCODE_F13: SDL_Scancode = 104;
    pub const SDL_SCANCODE_KP_EQUALS: SDL_Scancode = 103;
    /* *< The USB document says this is a status flag,
                               *   not a physical key - but some Mac keyboards
                               *   do have a power key. */
    pub const SDL_SCANCODE_POWER: SDL_Scancode = 102;
    /* *< windows contextual menu, compose */
    pub const SDL_SCANCODE_APPLICATION: SDL_Scancode = 101;
    /* *< This is the additional key that ISO
                                        *   keyboards have over ANSI ones,
                                        *   located between left shift and Y.
                                        *   Produces GRAVE ACCENT and TILDE in a
                                        *   US or UK Mac layout, REVERSE SOLIDUS
                                        *   (backslash) and VERTICAL LINE in a
                                        *   US or UK Windows layout, and
                                        *   LESS-THAN SIGN and GREATER-THAN SIGN
                                        *   in a Swiss German, German, or French
                                        *   layout. */
    pub const SDL_SCANCODE_NONUSBACKSLASH: SDL_Scancode = 100;
    pub const SDL_SCANCODE_KP_PERIOD: SDL_Scancode = 99;
    pub const SDL_SCANCODE_KP_0: SDL_Scancode = 98;
    pub const SDL_SCANCODE_KP_9: SDL_Scancode = 97;
    pub const SDL_SCANCODE_KP_8: SDL_Scancode = 96;
    pub const SDL_SCANCODE_KP_7: SDL_Scancode = 95;
    pub const SDL_SCANCODE_KP_6: SDL_Scancode = 94;
    pub const SDL_SCANCODE_KP_5: SDL_Scancode = 93;
    pub const SDL_SCANCODE_KP_4: SDL_Scancode = 92;
    pub const SDL_SCANCODE_KP_3: SDL_Scancode = 91;
    pub const SDL_SCANCODE_KP_2: SDL_Scancode = 90;
    pub const SDL_SCANCODE_KP_1: SDL_Scancode = 89;
    pub const SDL_SCANCODE_KP_ENTER: SDL_Scancode = 88;
    pub const SDL_SCANCODE_KP_PLUS: SDL_Scancode = 87;
    pub const SDL_SCANCODE_KP_MINUS: SDL_Scancode = 86;
    pub const SDL_SCANCODE_KP_MULTIPLY: SDL_Scancode = 85;
    pub const SDL_SCANCODE_KP_DIVIDE: SDL_Scancode = 84;
    /* *< num lock on PC, clear on Mac keyboards
                                     */
    pub const SDL_SCANCODE_NUMLOCKCLEAR: SDL_Scancode = 83;
    pub const SDL_SCANCODE_UP: SDL_Scancode = 82;
    pub const SDL_SCANCODE_DOWN: SDL_Scancode = 81;
    pub const SDL_SCANCODE_LEFT: SDL_Scancode = 80;
    pub const SDL_SCANCODE_RIGHT: SDL_Scancode = 79;
    pub const SDL_SCANCODE_PAGEDOWN: SDL_Scancode = 78;
    pub const SDL_SCANCODE_END: SDL_Scancode = 77;
    pub const SDL_SCANCODE_DELETE: SDL_Scancode = 76;
    pub const SDL_SCANCODE_PAGEUP: SDL_Scancode = 75;
    pub const SDL_SCANCODE_HOME: SDL_Scancode = 74;
    /* *< insert on PC, help on some Mac keyboards (but
                                   does send code 73, not 117) */
    pub const SDL_SCANCODE_INSERT: SDL_Scancode = 73;
    pub const SDL_SCANCODE_PAUSE: SDL_Scancode = 72;
    pub const SDL_SCANCODE_SCROLLLOCK: SDL_Scancode = 71;
    pub const SDL_SCANCODE_PRINTSCREEN: SDL_Scancode = 70;
    pub const SDL_SCANCODE_F12: SDL_Scancode = 69;
    pub const SDL_SCANCODE_F11: SDL_Scancode = 68;
    pub const SDL_SCANCODE_F10: SDL_Scancode = 67;
    pub const SDL_SCANCODE_F9: SDL_Scancode = 66;
    pub const SDL_SCANCODE_F8: SDL_Scancode = 65;
    pub const SDL_SCANCODE_F7: SDL_Scancode = 64;
    pub const SDL_SCANCODE_F6: SDL_Scancode = 63;
    pub const SDL_SCANCODE_F5: SDL_Scancode = 62;
    pub const SDL_SCANCODE_F4: SDL_Scancode = 61;
    pub const SDL_SCANCODE_F3: SDL_Scancode = 60;
    pub const SDL_SCANCODE_F2: SDL_Scancode = 59;
    pub const SDL_SCANCODE_F1: SDL_Scancode = 58;
    pub const SDL_SCANCODE_CAPSLOCK: SDL_Scancode = 57;
    pub const SDL_SCANCODE_SLASH: SDL_Scancode = 56;
    pub const SDL_SCANCODE_PERIOD: SDL_Scancode = 55;
    pub const SDL_SCANCODE_COMMA: SDL_Scancode = 54;
    /* *< Located in the top left corner (on both ANSI
                              *   and ISO keyboards). Produces GRAVE ACCENT and
                              *   TILDE in a US Windows layout and in US and UK
                              *   Mac layouts on ANSI keyboards, GRAVE ACCENT
                              *   and NOT SIGN in a UK Windows layout, SECTION
                              *   SIGN and PLUS-MINUS SIGN in US and UK Mac
                              *   layouts on ISO keyboards, SECTION SIGN and
                              *   DEGREE SIGN in a Swiss German layout (Mac:
                              *   only on ISO keyboards), CIRCUMFLEX ACCENT and
                              *   DEGREE SIGN in a German layout (Mac: only on
                              *   ISO keyboards), SUPERSCRIPT TWO and TILDE in a
                              *   French Windows layout, COMMERCIAL AT and
                              *   NUMBER SIGN in a French Mac layout on ISO
                              *   keyboards, and LESS-THAN SIGN and GREATER-THAN
                              *   SIGN in a Swiss German, German, or French Mac
                              *   layout on ANSI keyboards.
                              */
    pub const SDL_SCANCODE_GRAVE: SDL_Scancode = 53;
    pub const SDL_SCANCODE_APOSTROPHE: SDL_Scancode = 52;
    pub const SDL_SCANCODE_SEMICOLON: SDL_Scancode = 51;
    /* *< ISO USB keyboards actually use this code
                                  *   instead of 49 for the same key, but all
                                  *   OSes I've seen treat the two codes
                                  *   identically. So, as an implementor, unless
                                  *   your keyboard generates both of those
                                  *   codes and your OS treats them differently,
                                  *   you should generate SDL_SCANCODE_BACKSLASH
                                  *   instead of this code. As a user, you
                                  *   should not rely on this code because SDL
                                  *   will never generate it with most (all?)
                                  *   keyboards.
                                  */
    pub const SDL_SCANCODE_NONUSHASH: SDL_Scancode = 50;
    /* *< Located at the lower left of the return
                                  *   key on ISO keyboards and at the right end
                                  *   of the QWERTY row on ANSI keyboards.
                                  *   Produces REVERSE SOLIDUS (backslash) and
                                  *   VERTICAL LINE in a US layout, REVERSE
                                  *   SOLIDUS and VERTICAL LINE in a UK Mac
                                  *   layout, NUMBER SIGN and TILDE in a UK
                                  *   Windows layout, DOLLAR SIGN and POUND SIGN
                                  *   in a Swiss German layout, NUMBER SIGN and
                                  *   APOSTROPHE in a German layout, GRAVE
                                  *   ACCENT and POUND SIGN in a French Mac
                                  *   layout, and ASTERISK and MICRO SIGN in a
                                  *   French Windows layout.
                                  */
    pub const SDL_SCANCODE_BACKSLASH: SDL_Scancode = 49;
    pub const SDL_SCANCODE_RIGHTBRACKET: SDL_Scancode = 48;
    pub const SDL_SCANCODE_LEFTBRACKET: SDL_Scancode = 47;
    pub const SDL_SCANCODE_EQUALS: SDL_Scancode = 46;
    pub const SDL_SCANCODE_MINUS: SDL_Scancode = 45;
    pub const SDL_SCANCODE_SPACE: SDL_Scancode = 44;
    pub const SDL_SCANCODE_TAB: SDL_Scancode = 43;
    pub const SDL_SCANCODE_BACKSPACE: SDL_Scancode = 42;
    pub const SDL_SCANCODE_ESCAPE: SDL_Scancode = 41;
    pub const SDL_SCANCODE_RETURN: SDL_Scancode = 40;
    pub const SDL_SCANCODE_0: SDL_Scancode = 39;
    pub const SDL_SCANCODE_9: SDL_Scancode = 38;
    pub const SDL_SCANCODE_8: SDL_Scancode = 37;
    pub const SDL_SCANCODE_7: SDL_Scancode = 36;
    pub const SDL_SCANCODE_6: SDL_Scancode = 35;
    pub const SDL_SCANCODE_5: SDL_Scancode = 34;
    pub const SDL_SCANCODE_4: SDL_Scancode = 33;
    pub const SDL_SCANCODE_3: SDL_Scancode = 32;
    pub const SDL_SCANCODE_2: SDL_Scancode = 31;
    pub const SDL_SCANCODE_1: SDL_Scancode = 30;
    pub const SDL_SCANCODE_Z: SDL_Scancode = 29;
    pub const SDL_SCANCODE_Y: SDL_Scancode = 28;
    pub const SDL_SCANCODE_X: SDL_Scancode = 27;
    pub const SDL_SCANCODE_W: SDL_Scancode = 26;
    pub const SDL_SCANCODE_V: SDL_Scancode = 25;
    pub const SDL_SCANCODE_U: SDL_Scancode = 24;
    pub const SDL_SCANCODE_T: SDL_Scancode = 23;
    pub const SDL_SCANCODE_S: SDL_Scancode = 22;
    pub const SDL_SCANCODE_R: SDL_Scancode = 21;
    pub const SDL_SCANCODE_Q: SDL_Scancode = 20;
    pub const SDL_SCANCODE_P: SDL_Scancode = 19;
    pub const SDL_SCANCODE_O: SDL_Scancode = 18;
    pub const SDL_SCANCODE_N: SDL_Scancode = 17;
    pub const SDL_SCANCODE_M: SDL_Scancode = 16;
    pub const SDL_SCANCODE_L: SDL_Scancode = 15;
    pub const SDL_SCANCODE_K: SDL_Scancode = 14;
    pub const SDL_SCANCODE_J: SDL_Scancode = 13;
    pub const SDL_SCANCODE_I: SDL_Scancode = 12;
    pub const SDL_SCANCODE_H: SDL_Scancode = 11;
    pub const SDL_SCANCODE_G: SDL_Scancode = 10;
    pub const SDL_SCANCODE_F: SDL_Scancode = 9;
    pub const SDL_SCANCODE_E: SDL_Scancode = 8;
    pub const SDL_SCANCODE_D: SDL_Scancode = 7;
    pub const SDL_SCANCODE_C: SDL_Scancode = 6;
    pub const SDL_SCANCODE_B: SDL_Scancode = 5;
    /* *
     *  \name Usage page 0x07
     *
     *  These values are from usage page 0x07 (USB keyboard page).
     */
    /* @{ */
    pub const SDL_SCANCODE_A: SDL_Scancode = 4;
    pub const SDL_SCANCODE_UNKNOWN: SDL_Scancode = 0;
    use super::{libc};
}
#[header_src = "/usr/include/SDL2/SDL_keycode.h"]
pub mod SDL_keycode_h {
    /*
  Simple DirectMedia Layer
  Copyright (C) 1997-2018 Sam Lantinga <slouken@libsdl.org>

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.
*/
    /* *
 *  \file SDL_keycode.h
 *
 *  Defines constants which identify keyboard keys and modifiers.
 */
    /* *
 *  \brief The SDL virtual key representation.
 *
 *  Values of this type are used to represent keyboard keys using the current
 *  layout of the keyboard.  These values include Unicode values representing
 *  the unmodified character that would be generated by pressing the key, or
 *  an SDLK_* constant for those keys that do not generate characters.
 *
 *  A special exception is the number keys at the top of the keyboard which
 *  always map to SDLK_0...SDLK_9, regardless of layout.
 */
    pub type SDL_Keycode = Sint32;
    pub type unnamed_1 = libc::c_uint;
    pub const SDLK_AUDIOFASTFORWARD: unnamed_1 = 1073742110;
    pub const SDLK_AUDIOREWIND: unnamed_1 = 1073742109;
    pub const SDLK_APP2: unnamed_1 = 1073742108;
    pub const SDLK_APP1: unnamed_1 = 1073742107;
    pub const SDLK_SLEEP: unnamed_1 = 1073742106;
    pub const SDLK_EJECT: unnamed_1 = 1073742105;
    pub const SDLK_KBDILLUMUP: unnamed_1 = 1073742104;
    pub const SDLK_KBDILLUMDOWN: unnamed_1 = 1073742103;
    pub const SDLK_KBDILLUMTOGGLE: unnamed_1 = 1073742102;
    pub const SDLK_DISPLAYSWITCH: unnamed_1 = 1073742101;
    pub const SDLK_BRIGHTNESSUP: unnamed_1 = 1073742100;
    pub const SDLK_BRIGHTNESSDOWN: unnamed_1 = 1073742099;
    pub const SDLK_AC_BOOKMARKS: unnamed_1 = 1073742098;
    pub const SDLK_AC_REFRESH: unnamed_1 = 1073742097;
    pub const SDLK_AC_STOP: unnamed_1 = 1073742096;
    pub const SDLK_AC_FORWARD: unnamed_1 = 1073742095;
    pub const SDLK_AC_BACK: unnamed_1 = 1073742094;
    pub const SDLK_AC_HOME: unnamed_1 = 1073742093;
    pub const SDLK_AC_SEARCH: unnamed_1 = 1073742092;
    pub const SDLK_COMPUTER: unnamed_1 = 1073742091;
    pub const SDLK_CALCULATOR: unnamed_1 = 1073742090;
    pub const SDLK_MAIL: unnamed_1 = 1073742089;
    pub const SDLK_WWW: unnamed_1 = 1073742088;
    pub const SDLK_MEDIASELECT: unnamed_1 = 1073742087;
    pub const SDLK_AUDIOMUTE: unnamed_1 = 1073742086;
    pub const SDLK_AUDIOPLAY: unnamed_1 = 1073742085;
    pub const SDLK_AUDIOSTOP: unnamed_1 = 1073742084;
    pub const SDLK_AUDIOPREV: unnamed_1 = 1073742083;
    pub const SDLK_AUDIONEXT: unnamed_1 = 1073742082;
    pub const SDLK_MODE: unnamed_1 = 1073742081;
    pub const SDLK_RGUI: unnamed_1 = 1073742055;
    pub const SDLK_RALT: unnamed_1 = 1073742054;
    pub const SDLK_RSHIFT: unnamed_1 = 1073742053;
    pub const SDLK_RCTRL: unnamed_1 = 1073742052;
    pub const SDLK_LGUI: unnamed_1 = 1073742051;
    pub const SDLK_LALT: unnamed_1 = 1073742050;
    pub const SDLK_LSHIFT: unnamed_1 = 1073742049;
    pub const SDLK_LCTRL: unnamed_1 = 1073742048;
    pub const SDLK_KP_HEXADECIMAL: unnamed_1 = 1073742045;
    pub const SDLK_KP_DECIMAL: unnamed_1 = 1073742044;
    pub const SDLK_KP_OCTAL: unnamed_1 = 1073742043;
    pub const SDLK_KP_BINARY: unnamed_1 = 1073742042;
    pub const SDLK_KP_CLEARENTRY: unnamed_1 = 1073742041;
    pub const SDLK_KP_CLEAR: unnamed_1 = 1073742040;
    pub const SDLK_KP_PLUSMINUS: unnamed_1 = 1073742039;
    pub const SDLK_KP_MEMDIVIDE: unnamed_1 = 1073742038;
    pub const SDLK_KP_MEMMULTIPLY: unnamed_1 = 1073742037;
    pub const SDLK_KP_MEMSUBTRACT: unnamed_1 = 1073742036;
    pub const SDLK_KP_MEMADD: unnamed_1 = 1073742035;
    pub const SDLK_KP_MEMCLEAR: unnamed_1 = 1073742034;
    pub const SDLK_KP_MEMRECALL: unnamed_1 = 1073742033;
    pub const SDLK_KP_MEMSTORE: unnamed_1 = 1073742032;
    pub const SDLK_KP_EXCLAM: unnamed_1 = 1073742031;
    pub const SDLK_KP_AT: unnamed_1 = 1073742030;
    pub const SDLK_KP_SPACE: unnamed_1 = 1073742029;
    pub const SDLK_KP_HASH: unnamed_1 = 1073742028;
    pub const SDLK_KP_COLON: unnamed_1 = 1073742027;
    pub const SDLK_KP_DBLVERTICALBAR: unnamed_1 = 1073742026;
    pub const SDLK_KP_VERTICALBAR: unnamed_1 = 1073742025;
    pub const SDLK_KP_DBLAMPERSAND: unnamed_1 = 1073742024;
    pub const SDLK_KP_AMPERSAND: unnamed_1 = 1073742023;
    pub const SDLK_KP_GREATER: unnamed_1 = 1073742022;
    pub const SDLK_KP_LESS: unnamed_1 = 1073742021;
    pub const SDLK_KP_PERCENT: unnamed_1 = 1073742020;
    pub const SDLK_KP_POWER: unnamed_1 = 1073742019;
    pub const SDLK_KP_XOR: unnamed_1 = 1073742018;
    pub const SDLK_KP_F: unnamed_1 = 1073742017;
    pub const SDLK_KP_E: unnamed_1 = 1073742016;
    pub const SDLK_KP_D: unnamed_1 = 1073742015;
    pub const SDLK_KP_C: unnamed_1 = 1073742014;
    pub const SDLK_KP_B: unnamed_1 = 1073742013;
    pub const SDLK_KP_A: unnamed_1 = 1073742012;
    pub const SDLK_KP_BACKSPACE: unnamed_1 = 1073742011;
    pub const SDLK_KP_TAB: unnamed_1 = 1073742010;
    pub const SDLK_KP_RIGHTBRACE: unnamed_1 = 1073742009;
    pub const SDLK_KP_LEFTBRACE: unnamed_1 = 1073742008;
    pub const SDLK_KP_RIGHTPAREN: unnamed_1 = 1073742007;
    pub const SDLK_KP_LEFTPAREN: unnamed_1 = 1073742006;
    pub const SDLK_CURRENCYSUBUNIT: unnamed_1 = 1073742005;
    pub const SDLK_CURRENCYUNIT: unnamed_1 = 1073742004;
    pub const SDLK_DECIMALSEPARATOR: unnamed_1 = 1073742003;
    pub const SDLK_THOUSANDSSEPARATOR: unnamed_1 = 1073742002;
    pub const SDLK_KP_000: unnamed_1 = 1073742001;
    pub const SDLK_KP_00: unnamed_1 = 1073742000;
    pub const SDLK_EXSEL: unnamed_1 = 1073741988;
    pub const SDLK_CRSEL: unnamed_1 = 1073741987;
    pub const SDLK_CLEARAGAIN: unnamed_1 = 1073741986;
    pub const SDLK_OPER: unnamed_1 = 1073741985;
    pub const SDLK_OUT: unnamed_1 = 1073741984;
    pub const SDLK_SEPARATOR: unnamed_1 = 1073741983;
    pub const SDLK_RETURN2: unnamed_1 = 1073741982;
    pub const SDLK_PRIOR: unnamed_1 = 1073741981;
    pub const SDLK_CLEAR: unnamed_1 = 1073741980;
    pub const SDLK_CANCEL: unnamed_1 = 1073741979;
    pub const SDLK_SYSREQ: unnamed_1 = 1073741978;
    pub const SDLK_ALTERASE: unnamed_1 = 1073741977;
    pub const SDLK_KP_EQUALSAS400: unnamed_1 = 1073741958;
    pub const SDLK_KP_COMMA: unnamed_1 = 1073741957;
    pub const SDLK_VOLUMEDOWN: unnamed_1 = 1073741953;
    pub const SDLK_VOLUMEUP: unnamed_1 = 1073741952;
    pub const SDLK_MUTE: unnamed_1 = 1073741951;
    pub const SDLK_FIND: unnamed_1 = 1073741950;
    pub const SDLK_PASTE: unnamed_1 = 1073741949;
    pub const SDLK_COPY: unnamed_1 = 1073741948;
    pub const SDLK_CUT: unnamed_1 = 1073741947;
    pub const SDLK_UNDO: unnamed_1 = 1073741946;
    pub const SDLK_AGAIN: unnamed_1 = 1073741945;
    pub const SDLK_STOP: unnamed_1 = 1073741944;
    pub const SDLK_SELECT: unnamed_1 = 1073741943;
    pub const SDLK_MENU: unnamed_1 = 1073741942;
    pub const SDLK_HELP: unnamed_1 = 1073741941;
    pub const SDLK_EXECUTE: unnamed_1 = 1073741940;
    pub const SDLK_F24: unnamed_1 = 1073741939;
    pub const SDLK_F23: unnamed_1 = 1073741938;
    pub const SDLK_F22: unnamed_1 = 1073741937;
    pub const SDLK_F21: unnamed_1 = 1073741936;
    pub const SDLK_F20: unnamed_1 = 1073741935;
    pub const SDLK_F19: unnamed_1 = 1073741934;
    pub const SDLK_F18: unnamed_1 = 1073741933;
    pub const SDLK_F17: unnamed_1 = 1073741932;
    pub const SDLK_F16: unnamed_1 = 1073741931;
    pub const SDLK_F15: unnamed_1 = 1073741930;
    pub const SDLK_F14: unnamed_1 = 1073741929;
    pub const SDLK_F13: unnamed_1 = 1073741928;
    pub const SDLK_KP_EQUALS: unnamed_1 = 1073741927;
    pub const SDLK_POWER: unnamed_1 = 1073741926;
    pub const SDLK_APPLICATION: unnamed_1 = 1073741925;
    pub const SDLK_KP_PERIOD: unnamed_1 = 1073741923;
    pub const SDLK_KP_0: unnamed_1 = 1073741922;
    pub const SDLK_KP_9: unnamed_1 = 1073741921;
    pub const SDLK_KP_8: unnamed_1 = 1073741920;
    pub const SDLK_KP_7: unnamed_1 = 1073741919;
    pub const SDLK_KP_6: unnamed_1 = 1073741918;
    pub const SDLK_KP_5: unnamed_1 = 1073741917;
    pub const SDLK_KP_4: unnamed_1 = 1073741916;
    pub const SDLK_KP_3: unnamed_1 = 1073741915;
    pub const SDLK_KP_2: unnamed_1 = 1073741914;
    pub const SDLK_KP_1: unnamed_1 = 1073741913;
    pub const SDLK_KP_ENTER: unnamed_1 = 1073741912;
    pub const SDLK_KP_PLUS: unnamed_1 = 1073741911;
    pub const SDLK_KP_MINUS: unnamed_1 = 1073741910;
    pub const SDLK_KP_MULTIPLY: unnamed_1 = 1073741909;
    pub const SDLK_KP_DIVIDE: unnamed_1 = 1073741908;
    pub const SDLK_NUMLOCKCLEAR: unnamed_1 = 1073741907;
    pub const SDLK_UP: unnamed_1 = 1073741906;
    pub const SDLK_DOWN: unnamed_1 = 1073741905;
    pub const SDLK_LEFT: unnamed_1 = 1073741904;
    pub const SDLK_RIGHT: unnamed_1 = 1073741903;
    pub const SDLK_PAGEDOWN: unnamed_1 = 1073741902;
    pub const SDLK_END: unnamed_1 = 1073741901;
    pub const SDLK_DELETE: unnamed_1 = 127;
    pub const SDLK_PAGEUP: unnamed_1 = 1073741899;
    pub const SDLK_HOME: unnamed_1 = 1073741898;
    pub const SDLK_INSERT: unnamed_1 = 1073741897;
    pub const SDLK_PAUSE: unnamed_1 = 1073741896;
    pub const SDLK_SCROLLLOCK: unnamed_1 = 1073741895;
    pub const SDLK_PRINTSCREEN: unnamed_1 = 1073741894;
    pub const SDLK_F12: unnamed_1 = 1073741893;
    pub const SDLK_F11: unnamed_1 = 1073741892;
    pub const SDLK_F10: unnamed_1 = 1073741891;
    pub const SDLK_F9: unnamed_1 = 1073741890;
    pub const SDLK_F8: unnamed_1 = 1073741889;
    pub const SDLK_F7: unnamed_1 = 1073741888;
    pub const SDLK_F6: unnamed_1 = 1073741887;
    pub const SDLK_F5: unnamed_1 = 1073741886;
    pub const SDLK_F4: unnamed_1 = 1073741885;
    pub const SDLK_F3: unnamed_1 = 1073741884;
    pub const SDLK_F2: unnamed_1 = 1073741883;
    pub const SDLK_F1: unnamed_1 = 1073741882;
    pub const SDLK_CAPSLOCK: unnamed_1 = 1073741881;
    pub const SDLK_z: unnamed_1 = 122;
    pub const SDLK_y: unnamed_1 = 121;
    pub const SDLK_x: unnamed_1 = 120;
    pub const SDLK_w: unnamed_1 = 119;
    pub const SDLK_v: unnamed_1 = 118;
    pub const SDLK_u: unnamed_1 = 117;
    pub const SDLK_t: unnamed_1 = 116;
    pub const SDLK_s: unnamed_1 = 115;
    pub const SDLK_r: unnamed_1 = 114;
    pub const SDLK_q: unnamed_1 = 113;
    pub const SDLK_p: unnamed_1 = 112;
    pub const SDLK_o: unnamed_1 = 111;
    pub const SDLK_n: unnamed_1 = 110;
    pub const SDLK_m: unnamed_1 = 109;
    pub const SDLK_l: unnamed_1 = 108;
    pub const SDLK_k: unnamed_1 = 107;
    pub const SDLK_j: unnamed_1 = 106;
    pub const SDLK_i: unnamed_1 = 105;
    pub const SDLK_h: unnamed_1 = 104;
    pub const SDLK_g: unnamed_1 = 103;
    pub const SDLK_f: unnamed_1 = 102;
    pub const SDLK_e: unnamed_1 = 101;
    pub const SDLK_d: unnamed_1 = 100;
    pub const SDLK_c: unnamed_1 = 99;
    pub const SDLK_b: unnamed_1 = 98;
    pub const SDLK_a: unnamed_1 = 97;
    pub const SDLK_BACKQUOTE: unnamed_1 = 96;
    pub const SDLK_UNDERSCORE: unnamed_1 = 95;
    pub const SDLK_CARET: unnamed_1 = 94;
    pub const SDLK_RIGHTBRACKET: unnamed_1 = 93;
    pub const SDLK_BACKSLASH: unnamed_1 = 92;
    /*
       Skip uppercase letters
     */
    pub const SDLK_LEFTBRACKET: unnamed_1 = 91;
    pub const SDLK_AT: unnamed_1 = 64;
    pub const SDLK_QUESTION: unnamed_1 = 63;
    pub const SDLK_GREATER: unnamed_1 = 62;
    pub const SDLK_EQUALS: unnamed_1 = 61;
    pub const SDLK_LESS: unnamed_1 = 60;
    pub const SDLK_SEMICOLON: unnamed_1 = 59;
    pub const SDLK_COLON: unnamed_1 = 58;
    pub const SDLK_9: unnamed_1 = 57;
    pub const SDLK_8: unnamed_1 = 56;
    pub const SDLK_7: unnamed_1 = 55;
    pub const SDLK_6: unnamed_1 = 54;
    pub const SDLK_5: unnamed_1 = 53;
    pub const SDLK_4: unnamed_1 = 52;
    pub const SDLK_3: unnamed_1 = 51;
    pub const SDLK_2: unnamed_1 = 50;
    pub const SDLK_1: unnamed_1 = 49;
    pub const SDLK_0: unnamed_1 = 48;
    pub const SDLK_SLASH: unnamed_1 = 47;
    pub const SDLK_PERIOD: unnamed_1 = 46;
    pub const SDLK_MINUS: unnamed_1 = 45;
    pub const SDLK_COMMA: unnamed_1 = 44;
    pub const SDLK_PLUS: unnamed_1 = 43;
    pub const SDLK_ASTERISK: unnamed_1 = 42;
    pub const SDLK_RIGHTPAREN: unnamed_1 = 41;
    pub const SDLK_LEFTPAREN: unnamed_1 = 40;
    pub const SDLK_QUOTE: unnamed_1 = 39;
    pub const SDLK_AMPERSAND: unnamed_1 = 38;
    pub const SDLK_DOLLAR: unnamed_1 = 36;
    pub const SDLK_PERCENT: unnamed_1 = 37;
    pub const SDLK_HASH: unnamed_1 = 35;
    pub const SDLK_QUOTEDBL: unnamed_1 = 34;
    pub const SDLK_EXCLAIM: unnamed_1 = 33;
    pub const SDLK_SPACE: unnamed_1 = 32;
    pub const SDLK_TAB: unnamed_1 = 9;
    pub const SDLK_BACKSPACE: unnamed_1 = 8;
    pub const SDLK_ESCAPE: unnamed_1 = 27;
    pub const SDLK_RETURN: unnamed_1 = 13;
    pub const SDLK_UNKNOWN: unnamed_1 = 0;
    /* *
 * \brief Enumeration of valid key mods (possibly OR'd together).
 */
    pub type unnamed_2 = libc::c_uint;
    pub const KMOD_RESERVED: unnamed_2 = 32768;
    pub const KMOD_MODE: unnamed_2 = 16384;
    pub const KMOD_CAPS: unnamed_2 = 8192;
    pub const KMOD_NUM: unnamed_2 = 4096;
    pub const KMOD_RGUI: unnamed_2 = 2048;
    pub const KMOD_LGUI: unnamed_2 = 1024;
    pub const KMOD_RALT: unnamed_2 = 512;
    pub const KMOD_LALT: unnamed_2 = 256;
    pub const KMOD_RCTRL: unnamed_2 = 128;
    pub const KMOD_LCTRL: unnamed_2 = 64;
    pub const KMOD_RSHIFT: unnamed_2 = 2;
    pub const KMOD_LSHIFT: unnamed_2 = 1;
    pub const KMOD_NONE: unnamed_2 = 0;
    use super::SDL_stdinc_h::{Sint32};
    use super::{libc};
}
#[header_src = "/usr/include/SDL2/SDL_keyboard.h"]
pub mod SDL_keyboard_h {
    /*
  Simple DirectMedia Layer
  Copyright (C) 1997-2018 Sam Lantinga <slouken@libsdl.org>

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.
*/
    /* *
 *  \file SDL_keyboard.h
 *
 *  Include file for SDL keyboard event handling
 */
    /* Set up for C function definitions, even when using C++ */
    /* *
 *  \brief The SDL keysym structure, used in key events.
 *
 *  \note  If you are looking for translated character input, see the ::SDL_TEXTINPUT event.
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_Keysym {
        pub scancode: SDL_Scancode,
        pub sym: SDL_Keycode,
        pub mod_0: Uint16,
        pub unused: Uint32,
    }
    use super::SDL_scancode_h::{SDL_Scancode};
    use super::SDL_keycode_h::{SDL_Keycode};
    use super::SDL_stdinc_h::{Uint16, Uint32};
    use super::{libc};
    extern "C" {
        /* *
 *  \brief Get a human-readable name for a scancode.
 *
 *  \return A pointer to the name for the scancode.
 *          If the scancode doesn't have a name, this function returns
 *          an empty string ("").
 *
 *  \sa SDL_Scancode
 */
        #[no_mangle]
        pub fn SDL_GetScancodeName(scancode: SDL_Scancode)
         -> *const libc::c_char;
        /* *
 *  \brief Get a human-readable name for a key.
 *
 *  \return A pointer to a UTF-8 string that stays valid at least until the next
 *          call to this function. If you need it around any longer, you must
 *          copy it.  If the key doesn't have a name, this function returns an
 *          empty string ("").
 *
 *  \sa SDL_Keycode
 */
        #[no_mangle]
        pub fn SDL_GetKeyName(key: SDL_Keycode) -> *const libc::c_char;
        /* *
 *  \brief Start accepting Unicode text input events.
 *         This function will show the on-screen keyboard if supported.
 *
 *  \sa SDL_StopTextInput()
 *  \sa SDL_SetTextInputRect()
 *  \sa SDL_HasScreenKeyboardSupport()
 */
        #[no_mangle]
        pub fn SDL_StartTextInput();
        /* *
 *  \brief Stop receiving any text input events.
 *         This function will hide the on-screen keyboard if supported.
 *
 *  \sa SDL_StartTextInput()
 *  \sa SDL_HasScreenKeyboardSupport()
 */
        #[no_mangle]
        pub fn SDL_StopTextInput();
    }
}
#[header_src = "/usr/include/SDL2/SDL_joystick.h"]
pub mod SDL_joystick_h {
    pub type SDL_Joystick = _SDL_Joystick;
    /* *
 * This is a unique ID for a joystick for the time it is connected to the system,
 * and is never reused for the lifetime of the application. If the joystick is
 * disconnected and reconnected, it will get a new ID.
 *
 * The ID value starts at 0 and increments from there. The value -1 is an invalid ID.
 */
    pub type SDL_JoystickID = Sint32;
    use super::SDL_stdinc_h::{Sint32, Sint16, Uint8};
    use super::{libc};
    extern "C" {
        /*
  Simple DirectMedia Layer
  Copyright (C) 1997-2018 Sam Lantinga <slouken@libsdl.org>

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.
*/
        /* *
 *  \file SDL_joystick.h
 *
 *  Include file for SDL joystick event handling
 *
 * The term "device_index" identifies currently plugged in joystick devices between 0 and SDL_NumJoysticks(), with the exact joystick
 *   behind a device_index changing as joysticks are plugged and unplugged.
 *
 * The term "instance_id" is the current instantiation of a joystick device in the system, if the joystick is removed and then re-inserted
 *   then it will get a new instance_id, instance_id's are monotonically increasing identifiers of a joystick plugged in.
 *
 * The term JoystickGUID is a stable 128-bit identifier for a joystick device that does not change over time, it identifies class of
 *   the device (a X360 wired controller for example). This identifier is platform dependent.
 *
 *
 */
        /* Set up for C function definitions, even when using C++ */
        /* *
 *  \file SDL_joystick.h
 *
 *  In order to use these functions, SDL_Init() must have been called
 *  with the ::SDL_INIT_JOYSTICK flag.  This causes SDL to scan the system
 *  for joysticks, and load appropriate drivers.
 *
 *  If you would like to receive joystick updates while the application
 *  is in the background, you should set the following hint before calling
 *  SDL_Init(): SDL_HINT_JOYSTICK_ALLOW_BACKGROUND_EVENTS
 */
        /* *
 * The joystick structure used to identify an SDL joystick
 */
        pub type _SDL_Joystick;
        /* *
 *  Count the number of joysticks attached to the system right now
 */
        #[no_mangle]
        pub fn SDL_NumJoysticks() -> libc::c_int;
        /* *
 *  Get the implementation dependent name of a joystick.
 *  This can be called before any joysticks are opened.
 *  If no name can be found, this function returns NULL.
 */
        #[no_mangle]
        pub fn SDL_JoystickNameForIndex(device_index: libc::c_int)
         -> *const libc::c_char;
        /* *
 *  Open a joystick for use.
 *  The index passed as an argument refers to the N'th joystick on the system.
 *  This index is not the value which will identify this joystick in future
 *  joystick events.  The joystick's instance id (::SDL_JoystickID) will be used
 *  there instead.
 *
 *  \return A joystick identifier, or NULL if an error occurred.
 */
        #[no_mangle]
        pub fn SDL_JoystickOpen(device_index: libc::c_int)
         -> *mut SDL_Joystick;
        /* *
 *  Get the number of general axis controls on a joystick.
 */
        #[no_mangle]
        pub fn SDL_JoystickNumAxes(joystick: *mut SDL_Joystick)
         -> libc::c_int;
        /* *
 *  Get the number of trackballs on a joystick.
 *
 *  Joystick trackballs have only relative motion events associated
 *  with them and their state cannot be polled.
 */
        #[no_mangle]
        pub fn SDL_JoystickNumBalls(joystick: *mut SDL_Joystick)
         -> libc::c_int;
        /* *
 *  Get the number of POV hats on a joystick.
 */
        #[no_mangle]
        pub fn SDL_JoystickNumHats(joystick: *mut SDL_Joystick)
         -> libc::c_int;
        /* *
 *  Get the number of buttons on a joystick.
 */
        #[no_mangle]
        pub fn SDL_JoystickNumButtons(joystick: *mut SDL_Joystick)
         -> libc::c_int;
        /* *
 *  Update the current state of the open joysticks.
 *
 *  This is called automatically by the event loop if any joystick
 *  events are enabled.
 */
        #[no_mangle]
        pub fn SDL_JoystickUpdate();
        /* *
 *  Enable/disable joystick event polling.
 *
 *  If joystick events are disabled, you must call SDL_JoystickUpdate()
 *  yourself and check the state of the joystick when you want joystick
 *  information.
 *
 *  The state can be one of ::SDL_QUERY, ::SDL_ENABLE or ::SDL_IGNORE.
 */
        #[no_mangle]
        pub fn SDL_JoystickEventState(state: libc::c_int) -> libc::c_int;
        /* *
 *  Get the current state of an axis control on a joystick.
 *
 *  The state is a value ranging from -32768 to 32767.
 *
 *  The axis indices start at index 0.
 */
        #[no_mangle]
        pub fn SDL_JoystickGetAxis(joystick: *mut SDL_Joystick,
                                   axis: libc::c_int) -> Sint16;
        /* *
 *  \name Hat positions
 */
/* @{ */
        /* @} */
        /* *
 *  Get the current state of a POV hat on a joystick.
 *
 *  The hat indices start at index 0.
 *
 *  \return The return value is one of the following positions:
 *           - ::SDL_HAT_CENTERED
 *           - ::SDL_HAT_UP
 *           - ::SDL_HAT_RIGHT
 *           - ::SDL_HAT_DOWN
 *           - ::SDL_HAT_LEFT
 *           - ::SDL_HAT_RIGHTUP
 *           - ::SDL_HAT_RIGHTDOWN
 *           - ::SDL_HAT_LEFTUP
 *           - ::SDL_HAT_LEFTDOWN
 */
        #[no_mangle]
        pub fn SDL_JoystickGetHat(joystick: *mut SDL_Joystick,
                                  hat: libc::c_int) -> Uint8;
        /* *
 *  Get the ball axis change since the last poll.
 *
 *  \return 0, or -1 if you passed it invalid parameters.
 *
 *  The ball indices start at index 0.
 */
        #[no_mangle]
        pub fn SDL_JoystickGetBall(joystick: *mut SDL_Joystick,
                                   ball: libc::c_int, dx: *mut libc::c_int,
                                   dy: *mut libc::c_int) -> libc::c_int;
        /* *
 *  Get the current state of a button on a joystick.
 *
 *  The button indices start at index 0.
 */
        #[no_mangle]
        pub fn SDL_JoystickGetButton(joystick: *mut SDL_Joystick,
                                     button: libc::c_int) -> Uint8;
        /* *
 *  Close a joystick previously opened with SDL_JoystickOpen().
 */
        #[no_mangle]
        pub fn SDL_JoystickClose(joystick: *mut SDL_Joystick);
    }
}
#[header_src = "/usr/include/SDL2/SDL_gamecontroller.h"]
pub mod SDL_gamecontroller_h {
    pub type SDL_GameController = _SDL_GameController;
    /* *
 *  The list of axes available from a controller
 *
 *  Thumbstick axis values range from SDL_JOYSTICK_AXIS_MIN to SDL_JOYSTICK_AXIS_MAX,
 *  and are centered within ~8000 of zero, though advanced UI will allow users to set
 *  or autodetect the dead zone, which varies between controllers.
 *
 *  Trigger axis values range from 0 to SDL_JOYSTICK_AXIS_MAX.
 */
    pub type SDL_GameControllerAxis = libc::c_int;
    pub const SDL_CONTROLLER_AXIS_MAX: SDL_GameControllerAxis = 6;
    pub const SDL_CONTROLLER_AXIS_TRIGGERRIGHT: SDL_GameControllerAxis = 5;
    pub const SDL_CONTROLLER_AXIS_TRIGGERLEFT: SDL_GameControllerAxis = 4;
    pub const SDL_CONTROLLER_AXIS_RIGHTY: SDL_GameControllerAxis = 3;
    pub const SDL_CONTROLLER_AXIS_RIGHTX: SDL_GameControllerAxis = 2;
    pub const SDL_CONTROLLER_AXIS_LEFTY: SDL_GameControllerAxis = 1;
    pub const SDL_CONTROLLER_AXIS_LEFTX: SDL_GameControllerAxis = 0;
    pub const SDL_CONTROLLER_AXIS_INVALID: SDL_GameControllerAxis = -1;
    /* *
 *  The list of buttons available from a controller
 */
    pub type SDL_GameControllerButton = libc::c_int;
    pub const SDL_CONTROLLER_BUTTON_MAX: SDL_GameControllerButton = 15;
    pub const SDL_CONTROLLER_BUTTON_DPAD_RIGHT: SDL_GameControllerButton = 14;
    pub const SDL_CONTROLLER_BUTTON_DPAD_LEFT: SDL_GameControllerButton = 13;
    pub const SDL_CONTROLLER_BUTTON_DPAD_DOWN: SDL_GameControllerButton = 12;
    pub const SDL_CONTROLLER_BUTTON_DPAD_UP: SDL_GameControllerButton = 11;
    pub const SDL_CONTROLLER_BUTTON_RIGHTSHOULDER: SDL_GameControllerButton =
        10;
    pub const SDL_CONTROLLER_BUTTON_LEFTSHOULDER: SDL_GameControllerButton =
        9;
    pub const SDL_CONTROLLER_BUTTON_RIGHTSTICK: SDL_GameControllerButton = 8;
    pub const SDL_CONTROLLER_BUTTON_LEFTSTICK: SDL_GameControllerButton = 7;
    pub const SDL_CONTROLLER_BUTTON_START: SDL_GameControllerButton = 6;
    pub const SDL_CONTROLLER_BUTTON_GUIDE: SDL_GameControllerButton = 5;
    pub const SDL_CONTROLLER_BUTTON_BACK: SDL_GameControllerButton = 4;
    pub const SDL_CONTROLLER_BUTTON_Y: SDL_GameControllerButton = 3;
    pub const SDL_CONTROLLER_BUTTON_X: SDL_GameControllerButton = 2;
    pub const SDL_CONTROLLER_BUTTON_B: SDL_GameControllerButton = 1;
    pub const SDL_CONTROLLER_BUTTON_A: SDL_GameControllerButton = 0;
    pub const SDL_CONTROLLER_BUTTON_INVALID: SDL_GameControllerButton = -1;
    use super::{libc};
    use super::SDL_stdinc_h::{SDL_bool, Sint16, Uint8};
    extern "C" {
        /*
  Simple DirectMedia Layer
  Copyright (C) 1997-2018 Sam Lantinga <slouken@libsdl.org>

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.
*/
        /* *
 *  \file SDL_gamecontroller.h
 *
 *  Include file for SDL game controller event handling
 */
        /* Set up for C function definitions, even when using C++ */
        /* *
 *  \file SDL_gamecontroller.h
 *
 *  In order to use these functions, SDL_Init() must have been called
 *  with the ::SDL_INIT_GAMECONTROLLER flag.  This causes SDL to scan the system
 *  for game controllers, and load appropriate drivers.
 *
 *  If you would like to receive controller updates while the application
 *  is in the background, you should set the following hint before calling
 *  SDL_Init(): SDL_HINT_JOYSTICK_ALLOW_BACKGROUND_EVENTS
 */
        /* *
 * The gamecontroller structure used to identify an SDL game controller
 */
        pub type _SDL_GameController;
        /* *
 *  Is the joystick on this index supported by the game controller interface?
 */
        #[no_mangle]
        pub fn SDL_IsGameController(joystick_index: libc::c_int) -> SDL_bool;
        /* *
 *  Open a game controller for use.
 *  The index passed as an argument refers to the N'th game controller on the system.
 *  This index is not the value which will identify this controller in future
 *  controller events.  The joystick's instance id (::SDL_JoystickID) will be
 *  used there instead.
 *
 *  \return A controller identifier, or NULL if an error occurred.
 */
        #[no_mangle]
        pub fn SDL_GameControllerOpen(joystick_index: libc::c_int)
         -> *mut SDL_GameController;
        /* *
 *  Enable/disable controller event polling.
 *
 *  If controller events are disabled, you must call SDL_GameControllerUpdate()
 *  yourself and check the state of the controller when you want controller
 *  information.
 *
 *  The state can be one of ::SDL_QUERY, ::SDL_ENABLE or ::SDL_IGNORE.
 */
        #[no_mangle]
        pub fn SDL_GameControllerEventState(state: libc::c_int)
         -> libc::c_int;
        /* *
 *  Update the current state of the open game controllers.
 *
 *  This is called automatically by the event loop if any game controller
 *  events are enabled.
 */
        #[no_mangle]
        pub fn SDL_GameControllerUpdate();
        /* *
 *  Get the current state of an axis control on a game controller.
 *
 *  The state is a value ranging from -32768 to 32767 (except for the triggers,
 *  which range from 0 to 32767).
 *
 *  The axis indices start at index 0.
 */
        #[no_mangle]
        pub fn SDL_GameControllerGetAxis(gamecontroller:
                                             *mut SDL_GameController,
                                         axis: SDL_GameControllerAxis)
         -> Sint16;
        /* *
 *  Get the current state of a button on a game controller.
 *
 *  The button indices start at index 0.
 */
        #[no_mangle]
        pub fn SDL_GameControllerGetButton(gamecontroller:
                                               *mut SDL_GameController,
                                           button: SDL_GameControllerButton)
         -> Uint8;
        /* *
 *  Close a controller previously opened with SDL_GameControllerOpen().
 */
        #[no_mangle]
        pub fn SDL_GameControllerClose(gamecontroller:
                                           *mut SDL_GameController);
    }
}
#[header_src = "/usr/include/SDL2/SDL_touch.h"]
pub mod SDL_touch_h {
    /*
  Simple DirectMedia Layer
  Copyright (C) 1997-2018 Sam Lantinga <slouken@libsdl.org>

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.
*/
    /* *
 *  \file SDL_touch.h
 *
 *  Include file for SDL touch event handling.
 */
    /* Set up for C function definitions, even when using C++ */
    pub type SDL_TouchID = Sint64;
    pub type SDL_FingerID = Sint64;
    use super::SDL_stdinc_h::{Sint64};
}
#[header_src = "/usr/include/SDL2/SDL_gesture.h"]
pub mod SDL_gesture_h {
    /*
  Simple DirectMedia Layer
  Copyright (C) 1997-2018 Sam Lantinga <slouken@libsdl.org>

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.
*/
    /* *
 *  \file SDL_gesture.h
 *
 *  Include file for SDL gesture event handling.
 */
    /* Set up for C function definitions, even when using C++ */
    pub type SDL_GestureID = Sint64;
    use super::SDL_stdinc_h::{Sint64};
}
#[header_src = "/usr/include/SDL2/SDL_events.h"]
pub mod SDL_events_h {
    /*
  Simple DirectMedia Layer
  Copyright (C) 1997-2018 Sam Lantinga <slouken@libsdl.org>

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.
*/
    /* *
 *  \file SDL_events.h
 *
 *  Include file for SDL event handling.
 */
    /* Set up for C function definitions, even when using C++ */
    /* General keyboard/mouse state definitions */
    /* *
 * \brief The types of events that can be delivered.
 */
    pub type unnamed_3 = libc::c_uint;
    /* *
     *  This last event is only for bounding internal arrays
     */
    pub const SDL_LASTEVENT: unnamed_3 = 65535;
    /* * Events ::SDL_USEREVENT through ::SDL_LASTEVENT are for your use,
     *  and should be allocated with SDL_RegisterEvents()
     */
    pub const SDL_USEREVENT: unnamed_3 = 32768;
    /* *< The device has been reset and all textures need to be recreated */
    pub const SDL_RENDER_DEVICE_RESET: unnamed_3 = 8193;
    /* Render events */
    /* *< The render targets have been reset and their contents need to be updated */
    pub const SDL_RENDER_TARGETS_RESET: unnamed_3 = 8192;
    /* Sensor events */
    /* *< A sensor was updated */
    pub const SDL_SENSORUPDATE: unnamed_3 = 4608;
    /* *< An audio device has been removed. */
    pub const SDL_AUDIODEVICEREMOVED: unnamed_3 = 4353;
    /* Audio hotplug events */
    /* *< A new audio device is available */
    pub const SDL_AUDIODEVICEADDED: unnamed_3 = 4352;
    /* *< Current set of drops is now complete (NULL filename) */
    pub const SDL_DROPCOMPLETE: unnamed_3 = 4099;
    /* *< A new set of drops is beginning (NULL filename) */
    pub const SDL_DROPBEGIN: unnamed_3 = 4098;
    /* *< text/plain drag-and-drop event */
    pub const SDL_DROPTEXT: unnamed_3 = 4097;
    /* Drag and drop events */
    /* *< The system requests a file open */
    pub const SDL_DROPFILE: unnamed_3 = 4096;
    /* Clipboard events */
    /* *< The clipboard changed */
    pub const SDL_CLIPBOARDUPDATE: unnamed_3 = 2304;
    pub const SDL_MULTIGESTURE: unnamed_3 = 2050;
    pub const SDL_DOLLARRECORD: unnamed_3 = 2049;
    /* Gesture events */
    pub const SDL_DOLLARGESTURE: unnamed_3 = 2048;
    pub const SDL_FINGERMOTION: unnamed_3 = 1794;
    pub const SDL_FINGERUP: unnamed_3 = 1793;
    /* Touch events */
    pub const SDL_FINGERDOWN: unnamed_3 = 1792;
    /* *< The controller mapping was updated */
    pub const SDL_CONTROLLERDEVICEREMAPPED: unnamed_3 = 1621;
    /* *< An opened Game controller has been removed */
    pub const SDL_CONTROLLERDEVICEREMOVED: unnamed_3 = 1620;
    /* *< A new Game controller has been inserted into the system */
    pub const SDL_CONTROLLERDEVICEADDED: unnamed_3 = 1619;
    /* *< Game controller button released */
    pub const SDL_CONTROLLERBUTTONUP: unnamed_3 = 1618;
    /* *< Game controller button pressed */
    pub const SDL_CONTROLLERBUTTONDOWN: unnamed_3 = 1617;
    /* Game controller events */
    /* *< Game controller axis motion */
    pub const SDL_CONTROLLERAXISMOTION: unnamed_3 = 1616;
    /* *< An opened joystick has been removed */
    pub const SDL_JOYDEVICEREMOVED: unnamed_3 = 1542;
    /* *< A new joystick has been inserted into the system */
    pub const SDL_JOYDEVICEADDED: unnamed_3 = 1541;
    /* *< Joystick button released */
    pub const SDL_JOYBUTTONUP: unnamed_3 = 1540;
    /* *< Joystick button pressed */
    pub const SDL_JOYBUTTONDOWN: unnamed_3 = 1539;
    /* *< Joystick hat position change */
    pub const SDL_JOYHATMOTION: unnamed_3 = 1538;
    /* *< Joystick trackball motion */
    pub const SDL_JOYBALLMOTION: unnamed_3 = 1537;
    /* Joystick events */
    /* *< Joystick axis motion */
    pub const SDL_JOYAXISMOTION: unnamed_3 = 1536;
    /* *< Mouse wheel motion */
    pub const SDL_MOUSEWHEEL: unnamed_3 = 1027;
    /* *< Mouse button released */
    pub const SDL_MOUSEBUTTONUP: unnamed_3 = 1026;
    /* *< Mouse button pressed */
    pub const SDL_MOUSEBUTTONDOWN: unnamed_3 = 1025;
    /* Mouse events */
    /* *< Mouse moved */
    pub const SDL_MOUSEMOTION: unnamed_3 = 1024;
    /* *< Keymap changed due to a system event such as an
                                     input language or keyboard layout change.
                                */
    pub const SDL_KEYMAPCHANGED: unnamed_3 = 772;
    /* *< Keyboard text input */
    pub const SDL_TEXTINPUT: unnamed_3 = 771;
    /* *< Keyboard text editing (composition) */
    pub const SDL_TEXTEDITING: unnamed_3 = 770;
    /* *< Key released */
    pub const SDL_KEYUP: unnamed_3 = 769;
    /* Keyboard events */
    /* *< Key pressed */
    pub const SDL_KEYDOWN: unnamed_3 = 768;
    /* *< System specific event */
    pub const SDL_SYSWMEVENT: unnamed_3 = 513;
    /* Window events */
    /* *< Window state change */
    pub const SDL_WINDOWEVENT: unnamed_3 = 512;
    /* Display events */
    /* *< Display state change */
    pub const SDL_DISPLAYEVENT: unnamed_3 = 336;
    /* *< The application is now interactive
                                     Called on iOS in applicationDidBecomeActive()
                                     Called on Android in onResume()
                                */
    pub const SDL_APP_DIDENTERFOREGROUND: unnamed_3 = 262;
    /* *< The application is about to enter the foreground
                                     Called on iOS in applicationWillEnterForeground()
                                     Called on Android in onResume()
                                */
    pub const SDL_APP_WILLENTERFOREGROUND: unnamed_3 = 261;
    /* *< The application did enter the background and may not get CPU for some time
                                     Called on iOS in applicationDidEnterBackground()
                                     Called on Android in onPause()
                                */
    pub const SDL_APP_DIDENTERBACKGROUND: unnamed_3 = 260;
    /* *< The application is about to enter the background
                                     Called on iOS in applicationWillResignActive()
                                     Called on Android in onPause()
                                */
    pub const SDL_APP_WILLENTERBACKGROUND: unnamed_3 = 259;
    /* *< The application is low on memory, free memory if possible.
                                     Called on iOS in applicationDidReceiveMemoryWarning()
                                     Called on Android in onLowMemory()
                                */
    pub const SDL_APP_LOWMEMORY: unnamed_3 = 258;
    /* These application events have special meaning on iOS, see README-ios.md for details */
    /* *< The application is being terminated by the OS
                                     Called on iOS in applicationWillTerminate()
                                     Called on Android in onDestroy()
                                */
    pub const SDL_APP_TERMINATING: unnamed_3 = 257;
    /* Application events */
    /* *< User-requested quit */
    pub const SDL_QUIT: unnamed_3 = 256;
    /* *< Unused (do not remove) */
    pub const SDL_FIRSTEVENT: unnamed_3 = 0;
    /* *
 *  \brief Fields shared by every event
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_CommonEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
    }
    /* *
 *  \brief Display state change event data (event.display.*)
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_DisplayEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub display: Uint32,
        pub event: Uint8,
        pub padding1: Uint8,
        pub padding2: Uint8,
        pub padding3: Uint8,
        pub data1: Sint32,
    }
    /* *
 *  \brief Window state change event data (event.window.*)
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_WindowEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub windowID: Uint32,
        pub event: Uint8,
        pub padding1: Uint8,
        pub padding2: Uint8,
        pub padding3: Uint8,
        pub data1: Sint32,
        pub data2: Sint32,
    }
    /* *
 *  \brief Keyboard button event structure (event.key.*)
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_KeyboardEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub windowID: Uint32,
        pub state: Uint8,
        pub repeat: Uint8,
        pub padding2: Uint8,
        pub padding3: Uint8,
        pub keysym: SDL_Keysym,
    }
    /* *
 *  \brief Keyboard text editing event structure (event.edit.*)
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_TextEditingEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub windowID: Uint32,
        pub text: [libc::c_char; 32],
        pub start: Sint32,
        pub length: Sint32,
    }
    /* *
 *  \brief Keyboard text input event structure (event.text.*)
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_TextInputEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub windowID: Uint32,
        pub text: [libc::c_char; 32],
    }
    /* *
 *  \brief Mouse motion event structure (event.motion.*)
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_MouseMotionEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub windowID: Uint32,
        pub which: Uint32,
        pub state: Uint32,
        pub x: Sint32,
        pub y: Sint32,
        pub xrel: Sint32,
        pub yrel: Sint32,
    }
    /* *
 *  \brief Mouse button event structure (event.button.*)
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_MouseButtonEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub windowID: Uint32,
        pub which: Uint32,
        pub button: Uint8,
        pub state: Uint8,
        pub clicks: Uint8,
        pub padding1: Uint8,
        pub x: Sint32,
        pub y: Sint32,
    }
    /* *
 *  \brief Mouse wheel event structure (event.wheel.*)
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_MouseWheelEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub windowID: Uint32,
        pub which: Uint32,
        pub x: Sint32,
        pub y: Sint32,
        pub direction: Uint32,
    }
    /* *
 *  \brief Joystick axis motion event structure (event.jaxis.*)
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_JoyAxisEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub which: SDL_JoystickID,
        pub axis: Uint8,
        pub padding1: Uint8,
        pub padding2: Uint8,
        pub padding3: Uint8,
        pub value: Sint16,
        pub padding4: Uint16,
    }
    /* *
 *  \brief Joystick trackball motion event structure (event.jball.*)
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_JoyBallEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub which: SDL_JoystickID,
        pub ball: Uint8,
        pub padding1: Uint8,
        pub padding2: Uint8,
        pub padding3: Uint8,
        pub xrel: Sint16,
        pub yrel: Sint16,
    }
    /* *
 *  \brief Joystick hat position change event structure (event.jhat.*)
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_JoyHatEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub which: SDL_JoystickID,
        pub hat: Uint8,
        pub value: Uint8,
        pub padding1: Uint8,
        pub padding2: Uint8,
    }
    /* *
 *  \brief Joystick button event structure (event.jbutton.*)
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_JoyButtonEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub which: SDL_JoystickID,
        pub button: Uint8,
        pub state: Uint8,
        pub padding1: Uint8,
        pub padding2: Uint8,
    }
    /* *
 *  \brief Joystick device event structure (event.jdevice.*)
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_JoyDeviceEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub which: Sint32,
    }
    /* *
 *  \brief Game controller axis motion event structure (event.caxis.*)
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_ControllerAxisEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub which: SDL_JoystickID,
        pub axis: Uint8,
        pub padding1: Uint8,
        pub padding2: Uint8,
        pub padding3: Uint8,
        pub value: Sint16,
        pub padding4: Uint16,
    }
    /* *
 *  \brief Game controller button event structure (event.cbutton.*)
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_ControllerButtonEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub which: SDL_JoystickID,
        pub button: Uint8,
        pub state: Uint8,
        pub padding1: Uint8,
        pub padding2: Uint8,
    }
    /* *
 *  \brief Controller device event structure (event.cdevice.*)
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_ControllerDeviceEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub which: Sint32,
    }
    /* *
 *  \brief Audio device event structure (event.adevice.*)
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_AudioDeviceEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub which: Uint32,
        pub iscapture: Uint8,
        pub padding1: Uint8,
        pub padding2: Uint8,
        pub padding3: Uint8,
    }
    /* *
 *  \brief Touch finger event structure (event.tfinger.*)
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_TouchFingerEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub touchId: SDL_TouchID,
        pub fingerId: SDL_FingerID,
        pub x: libc::c_float,
        pub y: libc::c_float,
        pub dx: libc::c_float,
        pub dy: libc::c_float,
        pub pressure: libc::c_float,
    }
    /* *
 *  \brief Multiple Finger Gesture Event (event.mgesture.*)
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_MultiGestureEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub touchId: SDL_TouchID,
        pub dTheta: libc::c_float,
        pub dDist: libc::c_float,
        pub x: libc::c_float,
        pub y: libc::c_float,
        pub numFingers: Uint16,
        pub padding: Uint16,
    }
    /* *
 * \brief Dollar Gesture Event (event.dgesture.*)
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_DollarGestureEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub touchId: SDL_TouchID,
        pub gestureId: SDL_GestureID,
        pub numFingers: Uint32,
        pub error: libc::c_float,
        pub x: libc::c_float,
        pub y: libc::c_float,
    }
    /* *
 *  \brief An event used to request a file open by the system (event.drop.*)
 *         This event is enabled by default, you can disable it with SDL_EventState().
 *  \note If this event is enabled, you must free the filename in the event.
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_DropEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub file: *mut libc::c_char,
        pub windowID: Uint32,
    }
    /* *
 *  \brief Sensor event structure (event.sensor.*)
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_SensorEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub which: Sint32,
        pub data: [libc::c_float; 6],
    }
    /* *
 *  \brief The "quit requested" event
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_QuitEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
    }
    /* *
 *  \brief A user-defined event type (event.user.*)
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_UserEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub windowID: Uint32,
        pub code: Sint32,
        pub data1: *mut libc::c_void,
        pub data2: *mut libc::c_void,
    }
    /* *
 *  \brief A video driver dependent system event (event.syswm.*)
 *         This event is disabled by default, you can enable it with SDL_EventState()
 *
 *  \note If you want to use this event, you should include SDL_syswm.h.
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_SysWMEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub msg: *mut SDL_SysWMmsg,
    }
    /* *
 *  \brief General event structure
 */
    #[derive
    ( Copy , Clone )]
    #[repr
    ( C )]
    pub union SDL_Event {
        pub type_0: Uint32,
        pub common: SDL_CommonEvent,
        pub display: SDL_DisplayEvent,
        pub window: SDL_WindowEvent,
        pub key: SDL_KeyboardEvent,
        pub edit: SDL_TextEditingEvent,
        pub text: SDL_TextInputEvent,
        pub motion: SDL_MouseMotionEvent,
        pub button: SDL_MouseButtonEvent,
        pub wheel: SDL_MouseWheelEvent,
        pub jaxis: SDL_JoyAxisEvent,
        pub jball: SDL_JoyBallEvent,
        pub jhat: SDL_JoyHatEvent,
        pub jbutton: SDL_JoyButtonEvent,
        pub jdevice: SDL_JoyDeviceEvent,
        pub caxis: SDL_ControllerAxisEvent,
        pub cbutton: SDL_ControllerButtonEvent,
        pub cdevice: SDL_ControllerDeviceEvent,
        pub adevice: SDL_AudioDeviceEvent,
        pub sensor: SDL_SensorEvent,
        pub quit: SDL_QuitEvent,
        pub user: SDL_UserEvent,
        pub syswm: SDL_SysWMEvent,
        pub tfinger: SDL_TouchFingerEvent,
        pub mgesture: SDL_MultiGestureEvent,
        pub dgesture: SDL_DollarGestureEvent,
        pub drop: SDL_DropEvent,
        pub padding: [Uint8; 56],
    }
    /* @{ */
    pub type SDL_eventaction = libc::c_uint;
    pub const SDL_GETEVENT: SDL_eventaction = 2;
    pub const SDL_PEEKEVENT: SDL_eventaction = 1;
    pub const SDL_ADDEVENT: SDL_eventaction = 0;
    use super::{libc};
    use super::SDL_stdinc_h::{Uint32, Uint8, Sint32, Sint16, Uint16};
    use super::SDL_keyboard_h::{SDL_Keysym};
    use super::SDL_joystick_h::{SDL_JoystickID};
    use super::SDL_touch_h::{SDL_TouchID, SDL_FingerID};
    use super::SDL_gesture_h::{SDL_GestureID};
    extern "C" {
        pub type SDL_SysWMmsg;
        /* Function prototypes */
        /* *
 *  Pumps the event loop, gathering events from the input devices.
 *
 *  This function updates the event queue and internal input device state.
 *
 *  This should only be run in the thread that sets the video mode.
 */
        #[no_mangle]
        pub fn SDL_PumpEvents();
        /* *
 *  Checks the event queue for messages and optionally returns them.
 *
 *  If \c action is ::SDL_ADDEVENT, up to \c numevents events will be added to
 *  the back of the event queue.
 *
 *  If \c action is ::SDL_PEEKEVENT, up to \c numevents events at the front
 *  of the event queue, within the specified minimum and maximum type,
 *  will be returned and will not be removed from the queue.
 *
 *  If \c action is ::SDL_GETEVENT, up to \c numevents events at the front
 *  of the event queue, within the specified minimum and maximum type,
 *  will be returned and will be removed from the queue.
 *
 *  \return The number of events actually stored, or -1 if there was an error.
 *
 *  This function is thread-safe.
 */
        #[no_mangle]
        pub fn SDL_PeepEvents(events: *mut SDL_Event, numevents: libc::c_int,
                              action: SDL_eventaction, minType: Uint32,
                              maxType: Uint32) -> libc::c_int;
        /* *
 *  \brief Polls for currently pending events.
 *
 *  \return 1 if there are any pending events, or 0 if there are none available.
 *
 *  \param event If not NULL, the next event is removed from the queue and
 *               stored in that area.
 */
        #[no_mangle]
        pub fn SDL_PollEvent(event: *mut SDL_Event) -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/qcommon/q_shared.h"]
pub mod q_shared_h {
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
    // q_shared.h -- included first by ALL program modules.
// A user mod should never modify this file
    // Heartbeat for dpmaster protocol. You shouldn't change this unless you know what you're doing
    // When com_gamename is LEGACY_MASTER_GAMENAME, use quake3 master protocol.
// You shouldn't change this unless you know what you're doing
    // number of supported master servers
    // standard demo extension
    //Ignore __attribute__ on non-gcc platforms
    /* *********************************************************************
  VM Considerations

  The VM can not use the standard system headers because we aren't really
  using the compiler they were meant for.  We use bg_lib.h which contains
  prototypes for the functions we define for our own use in bg_lib.c.

  When writing mods, please add needed headers HERE, do not start including
  stuff like <stdio.h> in the various .c files that make up each of the VMs
  since you will be including system headers files can will have issues.

  Remember, if you use a C library function that is not defined in bg_lib.c,
  you will have to add your own version for support in the VM.

 **********************************************************************/
    //=============================================================
    pub type byte = libc::c_uchar;
    pub type qboolean = libc::c_uint;
    pub const qtrue: qboolean = 1;
    pub const qfalse: qboolean = 0;
    pub type qhandle_t = libc::c_int;
    pub type fileHandle_t = libc::c_int;
    // expand constants before stringifying them
    // angle indexes
    // up / down
    // left / right
    // fall over
    // the game guarantees that no string from the network will ever
// exceed MAX_STRING_CHARS
    // max length of a string passed to Cmd_TokenizeString
    // max tokens resulting from Cmd_TokenizeString
    // max length of an individual token
    // used for system info key only
    // max length of a quake game pathname
    // max length of a client name
    // parameters for command buffer stuffing
    pub type unnamed_4 = libc::c_uint;
    // add to end of the command buffer (normal case)
    pub const EXEC_APPEND: unnamed_4 = 2;
    // because some commands might cause the VM to be unloaded...
    // insert at current position, but don't run yet
    pub const EXEC_INSERT: unnamed_4 = 1;
    // don't return until completed, a VM should NEVER use this,
    pub const EXEC_NOW: unnamed_4 = 0;
    // parameters to the main Error routine
    pub type unnamed_5 = libc::c_uint;
    // pop up the need-cd dialog
    pub const ERR_NEED_CD: unnamed_5 = 4;
    // client disconnected from the server
    pub const ERR_DISCONNECT: unnamed_5 = 3;
    // don't kill server
    pub const ERR_SERVERDISCONNECT: unnamed_5 = 2;
    // print to console and disconnect from game
    pub const ERR_DROP: unnamed_5 = 1;
    // exit the entire game with a popup window
    pub const ERR_FATAL: unnamed_5 = 0;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct cvar_s {
        pub name: *mut libc::c_char,
        pub string: *mut libc::c_char,
        pub resetString: *mut libc::c_char,
        pub latchedString: *mut libc::c_char,
        pub flags: libc::c_int,
        pub modified: qboolean,
        pub modificationCount: libc::c_int,
        pub value: libc::c_float,
        pub integer: libc::c_int,
        pub validate: qboolean,
        pub integral: qboolean,
        pub min: libc::c_float,
        pub max: libc::c_float,
        pub description: *mut libc::c_char,
        pub next: *mut cvar_t,
        pub prev: *mut cvar_t,
        pub hashNext: *mut cvar_t,
        pub hashPrev: *mut cvar_t,
        pub hashIndex: libc::c_int,
    }
    /*
==========================================================

CVARS (console variables)

Many variables can be used for cheating purposes, so when
cheats is zero, force all unspecified variables to their
default values.
==========================================================
*/
    // set to cause it to be saved to vars.rc
    // used for system variables, not for player
					// specific configurations
    // sent to server on connect or change
    // sent in response to front end requests
    // these cvars will be duplicated on all clients
    // don't allow change from console at all,
    // but can be set from the command line
    // will only change when C code next does
    // a Cvar_Get(), so it can't be changed
					// without proper initialization.  modified
					// will be set, even though the value hasn't
					// changed yet
    // display only, cannot be set by user at all
    // created by a set command
    // can be set even when cheats are disabled, but is not archived
    // can not be changed if cheats are disabled
    // do not clear when a cvar_restart is issued
    // cvar was created by a server the client connected to.
    // cvar was created exclusively in one of the VMs.
    // prevent modifying this var from VMs or the server
    // These flags are only returned by the Cvar_Flags() function
    // Cvar was modified
    // Cvar doesn't exist.
    // nothing outside the Cvar_*() functions should modify these fields!
    pub type cvar_t = cvar_s;
    pub type connstate_t = libc::c_uint;
    // playing a cinematic or a static pic, not connected to a server
    pub const CA_CINEMATIC: connstate_t = 9;
    // game views should be displayed
    pub const CA_ACTIVE: connstate_t = 8;
    // got gamestate, waiting for first frame
    pub const CA_PRIMED: connstate_t = 7;
    // only during cgame initialization, never during main loop
    pub const CA_LOADING: connstate_t = 6;
    // netchan_t established, getting gamestate
    pub const CA_CONNECTED: connstate_t = 5;
    // sending challenge packets to the server
    pub const CA_CHALLENGING: connstate_t = 4;
    // sending request packets to the server
    pub const CA_CONNECTING: connstate_t = 3;
    // not used any more, was checking cd key 
    pub const CA_AUTHORIZING: connstate_t = 2;
    // not talking to a server
    pub const CA_DISCONNECTED: connstate_t = 1;
    pub const CA_UNINITIALIZED: connstate_t = 0;
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn COM_Parse(data_p: *mut *mut libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Com_HexStrToInt(str: *const libc::c_char) -> libc::c_int;
        // portable case insensitive compare
        #[no_mangle]
        pub fn Q_stricmp(s1: *const libc::c_char, s2: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn Q_strcat(dest: *mut libc::c_char, size: libc::c_int,
                        src: *const libc::c_char);
        // this is only here so the functions in q_shared.c and bg_*.c can link
        #[no_mangle]
        pub fn Com_Error(level: libc::c_int, error: *const libc::c_char, ...)
         -> !;
        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
}
#[header_src =
      "ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    //============================================================================
    /*
==============================================================

NET

==============================================================
*/
    // if this flag is set, always attempt ipv6 connections instead of ipv4 if a v6 address is found.
    // disables ipv6 multicast support if set.
    // number of old messages that must be kept on client and
    // server for delta comrpession and ping estimation
    // max number of usercmd_t in a packet
    // max string commands buffered for restransmit
    pub type netadrtype_t = libc::c_uint;
    pub const NA_UNSPEC: netadrtype_t = 7;
    pub const NA_MULTICAST6: netadrtype_t = 6;
    pub const NA_IP6: netadrtype_t = 5;
    pub const NA_IP: netadrtype_t = 4;
    pub const NA_BROADCAST: netadrtype_t = 3;
    pub const NA_LOOPBACK: netadrtype_t = 2;
    pub const NA_BOT: netadrtype_t = 1;
    // an address lookup failed
    pub const NA_BAD: netadrtype_t = 0;
    pub type netsrc_t = libc::c_uint;
    pub const NS_SERVER: netsrc_t = 1;
    pub const NS_CLIENT: netsrc_t = 0;
    // maximum length of an IPv6 address string including trailing '\0'
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct netadr_t {
        pub type_0: netadrtype_t,
        pub ip: [byte; 4],
        pub ip6: [byte; 16],
        pub port: libc::c_ushort,
        pub scope_id: libc::c_ulong,
    }
    // max length of a message, which may
    // be fragmented into multiple packets
    // ACK window of 48 download chunks. Cannot set this higher, or clients
    // will overflow the reliable commands buffer
    // 896 byte block chunks
    /*
Netchan handles packet fragmentation and out of order / duplicate suppression
*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct netchan_t {
        pub sock: netsrc_t,
        pub dropped: libc::c_int,
        pub remoteAddress: netadr_t,
        pub qport: libc::c_int,
        pub incomingSequence: libc::c_int,
        pub outgoingSequence: libc::c_int,
        pub fragmentSequence: libc::c_int,
        pub fragmentLength: libc::c_int,
        pub fragmentBuffer: [byte; 16384],
        pub unsentFragments: qboolean,
        pub unsentFragmentStart: libc::c_int,
        pub unsentLength: libc::c_int,
        pub unsentBuffer: [byte; 16384],
        pub challenge: libc::c_int,
        pub lastSentTime: libc::c_int,
        pub lastSentSize: libc::c_int,
        pub compat: qboolean,
    }
    // centralized and cleaned, that's the max string you can send to a Com_Printf / Com_DPrintf (above gets truncated)
    pub type sysEventType_t = libc::c_uint;
    // evPtr is a char*
    pub const SE_CONSOLE: sysEventType_t = 5;
    // evValue is an axis number and evValue2 is the current state (-127 to 127)
    pub const SE_JOYSTICK_AXIS: sysEventType_t = 4;
    // evValue and evValue2 are relative signed x / y moves
    pub const SE_MOUSE: sysEventType_t = 3;
    // evValue is an ascii char
    pub const SE_CHAR: sysEventType_t = 2;
    // evValue is a key code, evValue2 is the down flag
    pub const SE_KEY: sysEventType_t = 1;
    // SE_NONE must be zero
    // evTime is still valid
    pub const SE_NONE: sysEventType_t = 0;
    use super::{libc};
    use super::q_shared_h::{byte, qboolean, cvar_t};
    extern "C" {
        // allocates an initial text buffer that will grow as needed
        #[no_mangle]
        pub fn Cbuf_AddText(text: *const libc::c_char);
        // Adds command text at the end of the buffer, does NOT add a final \n
        #[no_mangle]
        pub fn Cbuf_ExecuteText(exec_when: libc::c_int,
                                text: *const libc::c_char);
        // Parses a single line of text into arguments and tries to execute it
// as if it was typed at the console
        /*
==============================================================

CVAR

==============================================================
*/
        /*

cvar_t variables are used to hold scalar or string variables that can be changed
or displayed at the console or prog code as well as accessed directly
in C code.

The user can access cvars from the console in three ways:
r_draworder			prints the current value
r_draworder 0		sets the current value to 0
set r_draworder 0	as above, but creates the cvar if not present

Cvars are restricted from having the same names as commands to keep this
interface from being ambiguous.

The are also occasionally used to communicated information between different
modules of the program.

*/
        #[no_mangle]
        pub fn Cvar_Get(var_name: *const libc::c_char,
                        value: *const libc::c_char, flags: libc::c_int)
         -> *mut cvar_t;
        // updates an interpreted modules' version of a cvar
        #[no_mangle]
        pub fn Cvar_Set(var_name: *const libc::c_char,
                        value: *const libc::c_char);
        // don't set the cvar immediately
        #[no_mangle]
        pub fn Cvar_SetValue(var_name: *const libc::c_char,
                             value: libc::c_float);
        #[no_mangle]
        pub fn Cvar_VariableIntegerValue(var_name: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn Com_QueueEvent(time: libc::c_int, type_0: sysEventType_t,
                              value: libc::c_int, value2: libc::c_int,
                              ptrLength: libc::c_int, ptr: *mut libc::c_void);
        #[no_mangle]
        pub fn Com_DPrintf(fmt: *const libc::c_char, ...);
        // Sys_Milliseconds should only be used for profiling purposes,
// any game related timing information should come from event timestamps
        #[no_mangle]
        pub fn Sys_Milliseconds() -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/sdl/sdl_input.c"]
pub mod sdl_input_c {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct unnamed_6 {
        pub buttons: [qboolean; 16],
        pub oldaxes: libc::c_uint,
        pub oldaaxes: [libc::c_int; 16],
        pub oldhats: libc::c_uint,
    }
    #[derive
    ( Copy , Clone )]
    #[repr
    ( C )]
    pub union unnamed_7 {
        pub key: keyNum_t,
        pub character: libc::c_int,
    }
    pub type consoleKey_t = consoleKey_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct consoleKey_s {
        pub type_0: unnamed_8,
        pub u: unnamed_7,
    }
    pub type unnamed_8 = libc::c_uint;
    pub const CHARACTER: unnamed_8 = 1;
    pub const QUAKE_KEY: unnamed_8 = 0;
    use super::q_shared_h::{qboolean};
    use super::{libc};
    use super::keycodes_h::{keyNum_t};
}
#[header_src =
      "ioq3/code/renderercommon/tr_types.h"]
pub mod tr_types_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct glconfig_t {
        pub renderer_string: [libc::c_char; 1024],
        pub vendor_string: [libc::c_char; 1024],
        pub version_string: [libc::c_char; 1024],
        pub extensions_string: [libc::c_char; 8192],
        pub maxTextureSize: libc::c_int,
        pub numTextureUnits: libc::c_int,
        pub colorBits: libc::c_int,
        pub depthBits: libc::c_int,
        pub stencilBits: libc::c_int,
        pub driverType: glDriverType_t,
        pub hardwareType: glHardwareType_t,
        pub deviceSupportsGamma: qboolean,
        pub textureCompression: textureCompression_t,
        pub textureEnvAddAvailable: qboolean,
        pub vidWidth: libc::c_int,
        pub vidHeight: libc::c_int,
        pub windowAspect: libc::c_float,
        pub displayFrequency: libc::c_int,
        pub isFullscreen: qboolean,
        pub stereoEnabled: qboolean,
        pub smpActive: qboolean,
    }
    /*
** glconfig_t
**
** Contains variables specific to the OpenGL configuration
** being run right now.  These are constant once the OpenGL
** subsystem is initialized.
*/
    pub type textureCompression_t = libc::c_uint;
    // this is for the GL_EXT_texture_compression_s3tc extension.
    pub const TC_S3TC_ARB: textureCompression_t = 2;
    // this is for the GL_S3_s3tc extension.
    pub const TC_S3TC: textureCompression_t = 1;
    pub const TC_NONE: textureCompression_t = 0;
    pub type glHardwareType_t = libc::c_uint;
    // where you don't have src*dst
    pub const GLHW_PERMEDIA2: glHardwareType_t = 4;
    // where you can't modulate alpha on alpha textures
    pub const GLHW_RAGEPRO: glHardwareType_t = 3;
    // the hardware type then there can NOT exist a secondary
							// display adapter
    // where you can't interpolate alpha
    pub const GLHW_RIVA128: glHardwareType_t = 2;
    // Voodoo Banshee or Voodoo3, relevant since if this is
    pub const GLHW_3DFX_2D3D: glHardwareType_t = 1;
    // where everything works the way it should
    pub const GLHW_GENERIC: glHardwareType_t = 0;
    pub type glDriverType_t = libc::c_uint;
    // driver is a 3Dfx standalone driver
    pub const GLDRV_VOODOO: glDriverType_t = 2;
    // WARNING: there are tests that check for
								// > GLDRV_ICD for minidriverness, so this
								// should always be the lowest value in this
								// enum set
    // driver is a non-3Dfx standalone driver
    pub const GLDRV_STANDALONE: glDriverType_t = 1;
    // driver is integrated with window system
    pub const GLDRV_ICD: glDriverType_t = 0;
    use super::{libc};
    use super::q_shared_h::{qboolean};
}
#[header_src =
      "ioq3/code/client/client.h"]
pub mod client_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct clientStatic_t {
        pub cddialog: qboolean,
        pub rendererStarted: qboolean,
        pub soundStarted: qboolean,
        pub soundRegistered: qboolean,
        pub uiStarted: qboolean,
        pub cgameStarted: qboolean,
        pub framecount: libc::c_int,
        pub frametime: libc::c_int,
        pub realtime: libc::c_int,
        pub realFrametime: libc::c_int,
        pub numlocalservers: libc::c_int,
        pub localServers: [serverInfo_t; 128],
        pub numglobalservers: libc::c_int,
        pub globalServers: [serverInfo_t; 4096],
        pub numGlobalServerAddresses: libc::c_int,
        pub globalServerAddresses: [netadr_t; 4096],
        pub numfavoriteservers: libc::c_int,
        pub favoriteServers: [serverInfo_t; 128],
        pub pingUpdateSource: libc::c_int,
        pub updateServer: netadr_t,
        pub updateChallenge: [libc::c_char; 1024],
        pub updateInfoString: [libc::c_char; 1024],
        pub authorizeServer: netadr_t,
        pub rconAddress: netadr_t,
        pub glconfig: glconfig_t,
        pub charSetShader: qhandle_t,
        pub whiteShader: qhandle_t,
        pub consoleShader: qhandle_t,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct serverInfo_t {
        pub adr: netadr_t,
        pub hostName: [libc::c_char; 32],
        pub mapName: [libc::c_char; 32],
        pub game: [libc::c_char; 32],
        pub netType: libc::c_int,
        pub gameType: libc::c_int,
        pub clients: libc::c_int,
        pub maxClients: libc::c_int,
        pub minPing: libc::c_int,
        pub maxPing: libc::c_int,
        pub ping: libc::c_int,
        pub visible: qboolean,
        pub punkbuster: libc::c_int,
        pub g_humanplayers: libc::c_int,
        pub g_needpass: libc::c_int,
    }
    /*
=============================================================================

the clientConnection_t structure is wiped when disconnecting from a server,
either to go to a full screen console, play a demo, or connect to a different server

A connection can be to either a server through the network layer or a
demo through a file.

=============================================================================
*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct clientConnection_t {
        pub state: connstate_t,
        pub clientNum: libc::c_int,
        pub lastPacketSentTime: libc::c_int,
        pub lastPacketTime: libc::c_int,
        pub servername: [libc::c_char; 4096],
        pub serverAddress: netadr_t,
        pub connectTime: libc::c_int,
        pub connectPacketCount: libc::c_int,
        pub serverMessage: [libc::c_char; 1024],
        pub challenge: libc::c_int,
        pub checksumFeed: libc::c_int,
        pub reliableSequence: libc::c_int,
        pub reliableAcknowledge: libc::c_int,
        pub reliableCommands: [[libc::c_char; 1024]; 64],
        pub serverMessageSequence: libc::c_int,
        pub serverCommandSequence: libc::c_int,
        pub lastExecutedServerCommand: libc::c_int,
        pub serverCommands: [[libc::c_char; 1024]; 64],
        pub download: fileHandle_t,
        pub downloadTempName: [libc::c_char; 4096],
        pub downloadName: [libc::c_char; 4096],
        pub cURLEnabled: qboolean,
        pub cURLUsed: qboolean,
        pub cURLDisconnected: qboolean,
        pub downloadURL: [libc::c_char; 4096],
        pub downloadCURL: *mut libc::c_void,
        pub downloadCURLM: *mut libc::c_void,
        pub sv_allowDownload: libc::c_int,
        pub sv_dlURL: [libc::c_char; 256],
        pub downloadNumber: libc::c_int,
        pub downloadBlock: libc::c_int,
        pub downloadCount: libc::c_int,
        pub downloadSize: libc::c_int,
        pub downloadList: [libc::c_char; 1024],
        pub downloadRestart: qboolean,
        pub demoName: [libc::c_char; 64],
        pub spDemoRecording: qboolean,
        pub demorecording: qboolean,
        pub demoplaying: qboolean,
        pub demowaiting: qboolean,
        pub firstDemoFrameSkipped: qboolean,
        pub demofile: fileHandle_t,
        pub timeDemoFrames: libc::c_int,
        pub timeDemoStart: libc::c_int,
        pub timeDemoBaseTime: libc::c_int,
        pub timeDemoLastFrame: libc::c_int,
        pub timeDemoMinDuration: libc::c_int,
        pub timeDemoMaxDuration: libc::c_int,
        pub timeDemoDurations: [libc::c_uchar; 4096],
        pub aviVideoFrameRemainder: libc::c_float,
        pub aviSoundFrameRemainder: libc::c_float,
        pub voipEnabled: qboolean,
        pub voipCodecInitialized: qboolean,
        pub opusDecoder: [*mut OpusDecoder; 64],
        pub voipIncomingGeneration: [byte; 64],
        pub voipIncomingSequence: [libc::c_int; 64],
        pub voipGain: [libc::c_float; 64],
        pub voipIgnore: [qboolean; 64],
        pub voipMuteAll: qboolean,
        pub voipTargets: [uint8_t; 8],
        pub voipFlags: uint8_t,
        pub opusEncoder: *mut OpusEncoder,
        pub voipOutgoingDataSize: libc::c_int,
        pub voipOutgoingDataFrames: libc::c_int,
        pub voipOutgoingSequence: libc::c_int,
        pub voipOutgoingGeneration: byte,
        pub voipOutgoingData: [byte; 1024],
        pub voipPower: libc::c_float,
        pub compat: qboolean,
        pub netchan: netchan_t,
    }
    use super::q_shared_h::{qboolean, qhandle_t, connstate_t, fileHandle_t,
                            byte, cvar_t};
    use super::{libc};
    use super::qcommon_h::{netadr_t, netchan_t};
    use super::tr_types_h::{glconfig_t};
    use super::curl_h::{CURL};
    use super::multi_h::{CURLM};
    use super::opus_h::{OpusDecoder, OpusEncoder};
    use super::stdint_uintn_h::{uint8_t};
    extern "C" {
        #[no_mangle]
        pub static mut cls: clientStatic_t;
        #[no_mangle]
        pub fn Key_StringToKeynum(str: *mut libc::c_char) -> libc::c_int;
        #[no_mangle]
        pub static mut cl_consoleKeys: *mut cvar_t;
        #[no_mangle]
        pub fn Key_KeynumToString(keynum: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Key_GetCatcher() -> libc::c_int;
        #[no_mangle]
        pub static mut clc: clientConnection_t;
        #[no_mangle]
        pub static mut j_up: *mut cvar_t;
        #[no_mangle]
        pub static mut j_up_axis: *mut cvar_t;
        #[no_mangle]
        pub static mut j_yaw: *mut cvar_t;
        #[no_mangle]
        pub static mut j_yaw_axis: *mut cvar_t;
        #[no_mangle]
        pub static mut j_pitch: *mut cvar_t;
        #[no_mangle]
        pub static mut j_pitch_axis: *mut cvar_t;
        #[no_mangle]
        pub static mut j_side: *mut cvar_t;
        #[no_mangle]
        pub static mut j_side_axis: *mut cvar_t;
        #[no_mangle]
        pub static mut j_forward: *mut cvar_t;
        #[no_mangle]
        pub static mut j_forward_axis: *mut cvar_t;
    }
}
#[header_src =
      "ioq3/code/client/keycodes.h"]
pub mod keycodes_h {
    pub const K_MWHEELDOWN: keyNum_t = 183;
    pub const K_MWHEELUP: keyNum_t = 184;
    pub const K_AUX1: keyNum_t = 217;
    pub const K_MOUSE5: keyNum_t = 182;
    pub const K_MOUSE4: keyNum_t = 181;
    pub const K_MOUSE2: keyNum_t = 179;
    pub const K_MOUSE3: keyNum_t = 180;
    pub const K_MOUSE1: keyNum_t = 178;
    // Pseudo-key that brings the console down
    pub const K_CONSOLE: keyNum_t = 365;
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
    //
// these are the key numbers that should be passed to KeyEvent
//
    // normal keys should be passed as lowercased ascii
    pub type keyNum_t = libc::c_uint;
    pub const MAX_KEYS: keyNum_t = 366;
    pub const K_PAD0_RIGHTTRIGGER: keyNum_t = 364;
    pub const K_PAD0_LEFTTRIGGER: keyNum_t = 363;
    pub const K_PAD0_RIGHTSTICK_DOWN: keyNum_t = 362;
    pub const K_PAD0_RIGHTSTICK_UP: keyNum_t = 361;
    pub const K_PAD0_RIGHTSTICK_RIGHT: keyNum_t = 360;
    pub const K_PAD0_RIGHTSTICK_LEFT: keyNum_t = 359;
    pub const K_PAD0_LEFTSTICK_DOWN: keyNum_t = 358;
    pub const K_PAD0_LEFTSTICK_UP: keyNum_t = 357;
    pub const K_PAD0_LEFTSTICK_RIGHT: keyNum_t = 356;
    pub const K_PAD0_LEFTSTICK_LEFT: keyNum_t = 355;
    pub const K_PAD0_DPAD_RIGHT: keyNum_t = 354;
    pub const K_PAD0_DPAD_LEFT: keyNum_t = 353;
    pub const K_PAD0_DPAD_DOWN: keyNum_t = 352;
    pub const K_PAD0_DPAD_UP: keyNum_t = 351;
    pub const K_PAD0_RIGHTSHOULDER: keyNum_t = 350;
    pub const K_PAD0_LEFTSHOULDER: keyNum_t = 349;
    pub const K_PAD0_RIGHTSTICK_CLICK: keyNum_t = 348;
    pub const K_PAD0_LEFTSTICK_CLICK: keyNum_t = 347;
    pub const K_PAD0_START: keyNum_t = 346;
    pub const K_PAD0_GUIDE: keyNum_t = 345;
    pub const K_PAD0_BACK: keyNum_t = 344;
    pub const K_PAD0_Y: keyNum_t = 343;
    pub const K_PAD0_X: keyNum_t = 342;
    pub const K_PAD0_B: keyNum_t = 341;
    // Gamepad controls
	// Ordered to match SDL2 game controller buttons and axes
	// Do not change this order without also changing IN_GamepadMove() in SDL_input.c
    pub const K_PAD0_A: keyNum_t = 340;
    pub const K_UNDO: keyNum_t = 339;
    pub const K_EURO: keyNum_t = 338;
    pub const K_MENU: keyNum_t = 337;
    pub const K_BREAK: keyNum_t = 336;
    pub const K_SCROLLOCK: keyNum_t = 335;
    pub const K_SYSREQ: keyNum_t = 334;
    pub const K_PRINT: keyNum_t = 333;
    pub const K_HELP: keyNum_t = 332;
    pub const K_MODE: keyNum_t = 331;
    pub const K_COMPOSE: keyNum_t = 330;
    pub const K_SUPER: keyNum_t = 329;
    pub const K_WORLD_95: keyNum_t = 328;
    pub const K_WORLD_94: keyNum_t = 327;
    pub const K_WORLD_93: keyNum_t = 326;
    pub const K_WORLD_92: keyNum_t = 325;
    pub const K_WORLD_91: keyNum_t = 324;
    pub const K_WORLD_90: keyNum_t = 323;
    pub const K_WORLD_89: keyNum_t = 322;
    pub const K_WORLD_88: keyNum_t = 321;
    pub const K_WORLD_87: keyNum_t = 320;
    pub const K_WORLD_86: keyNum_t = 319;
    pub const K_WORLD_85: keyNum_t = 318;
    pub const K_WORLD_84: keyNum_t = 317;
    pub const K_WORLD_83: keyNum_t = 316;
    pub const K_WORLD_82: keyNum_t = 315;
    pub const K_WORLD_81: keyNum_t = 314;
    pub const K_WORLD_80: keyNum_t = 313;
    pub const K_WORLD_79: keyNum_t = 312;
    pub const K_WORLD_78: keyNum_t = 311;
    pub const K_WORLD_77: keyNum_t = 310;
    pub const K_WORLD_76: keyNum_t = 309;
    pub const K_WORLD_75: keyNum_t = 308;
    pub const K_WORLD_74: keyNum_t = 307;
    pub const K_WORLD_73: keyNum_t = 306;
    pub const K_WORLD_72: keyNum_t = 305;
    pub const K_WORLD_71: keyNum_t = 304;
    pub const K_WORLD_70: keyNum_t = 303;
    pub const K_WORLD_69: keyNum_t = 302;
    pub const K_WORLD_68: keyNum_t = 301;
    pub const K_WORLD_67: keyNum_t = 300;
    pub const K_WORLD_66: keyNum_t = 299;
    pub const K_WORLD_65: keyNum_t = 298;
    pub const K_WORLD_64: keyNum_t = 297;
    pub const K_WORLD_63: keyNum_t = 296;
    pub const K_WORLD_62: keyNum_t = 295;
    pub const K_WORLD_61: keyNum_t = 294;
    pub const K_WORLD_60: keyNum_t = 293;
    pub const K_WORLD_59: keyNum_t = 292;
    pub const K_WORLD_58: keyNum_t = 291;
    pub const K_WORLD_57: keyNum_t = 290;
    pub const K_WORLD_56: keyNum_t = 289;
    pub const K_WORLD_55: keyNum_t = 288;
    pub const K_WORLD_54: keyNum_t = 287;
    pub const K_WORLD_53: keyNum_t = 286;
    pub const K_WORLD_52: keyNum_t = 285;
    pub const K_WORLD_51: keyNum_t = 284;
    pub const K_WORLD_50: keyNum_t = 283;
    pub const K_WORLD_49: keyNum_t = 282;
    pub const K_WORLD_48: keyNum_t = 281;
    pub const K_WORLD_47: keyNum_t = 280;
    pub const K_WORLD_46: keyNum_t = 279;
    pub const K_WORLD_45: keyNum_t = 278;
    pub const K_WORLD_44: keyNum_t = 277;
    pub const K_WORLD_43: keyNum_t = 276;
    pub const K_WORLD_42: keyNum_t = 275;
    pub const K_WORLD_41: keyNum_t = 274;
    pub const K_WORLD_40: keyNum_t = 273;
    pub const K_WORLD_39: keyNum_t = 272;
    pub const K_WORLD_38: keyNum_t = 271;
    pub const K_WORLD_37: keyNum_t = 270;
    pub const K_WORLD_36: keyNum_t = 269;
    pub const K_WORLD_35: keyNum_t = 268;
    pub const K_WORLD_34: keyNum_t = 267;
    pub const K_WORLD_33: keyNum_t = 266;
    pub const K_WORLD_32: keyNum_t = 265;
    pub const K_WORLD_31: keyNum_t = 264;
    pub const K_WORLD_30: keyNum_t = 263;
    pub const K_WORLD_29: keyNum_t = 262;
    pub const K_WORLD_28: keyNum_t = 261;
    pub const K_WORLD_27: keyNum_t = 260;
    pub const K_WORLD_26: keyNum_t = 259;
    pub const K_WORLD_25: keyNum_t = 258;
    pub const K_WORLD_24: keyNum_t = 257;
    pub const K_WORLD_23: keyNum_t = 256;
    pub const K_WORLD_22: keyNum_t = 255;
    pub const K_WORLD_21: keyNum_t = 254;
    pub const K_WORLD_20: keyNum_t = 253;
    pub const K_WORLD_19: keyNum_t = 252;
    pub const K_WORLD_18: keyNum_t = 251;
    pub const K_WORLD_17: keyNum_t = 250;
    pub const K_WORLD_16: keyNum_t = 249;
    pub const K_WORLD_15: keyNum_t = 248;
    pub const K_WORLD_14: keyNum_t = 247;
    pub const K_WORLD_13: keyNum_t = 246;
    pub const K_WORLD_12: keyNum_t = 245;
    pub const K_WORLD_11: keyNum_t = 244;
    pub const K_WORLD_10: keyNum_t = 243;
    pub const K_WORLD_9: keyNum_t = 242;
    pub const K_WORLD_8: keyNum_t = 241;
    pub const K_WORLD_7: keyNum_t = 240;
    pub const K_WORLD_6: keyNum_t = 239;
    pub const K_WORLD_5: keyNum_t = 238;
    pub const K_WORLD_4: keyNum_t = 237;
    pub const K_WORLD_3: keyNum_t = 236;
    pub const K_WORLD_2: keyNum_t = 235;
    pub const K_WORLD_1: keyNum_t = 234;
    pub const K_WORLD_0: keyNum_t = 233;
    pub const K_AUX16: keyNum_t = 232;
    pub const K_AUX15: keyNum_t = 231;
    pub const K_AUX14: keyNum_t = 230;
    pub const K_AUX13: keyNum_t = 229;
    pub const K_AUX12: keyNum_t = 228;
    pub const K_AUX11: keyNum_t = 227;
    pub const K_AUX10: keyNum_t = 226;
    pub const K_AUX9: keyNum_t = 225;
    pub const K_AUX8: keyNum_t = 224;
    pub const K_AUX7: keyNum_t = 223;
    pub const K_AUX6: keyNum_t = 222;
    pub const K_AUX5: keyNum_t = 221;
    pub const K_AUX4: keyNum_t = 220;
    pub const K_AUX3: keyNum_t = 219;
    pub const K_AUX2: keyNum_t = 218;
    pub const K_JOY32: keyNum_t = 216;
    pub const K_JOY31: keyNum_t = 215;
    pub const K_JOY30: keyNum_t = 214;
    pub const K_JOY29: keyNum_t = 213;
    pub const K_JOY28: keyNum_t = 212;
    pub const K_JOY27: keyNum_t = 211;
    pub const K_JOY26: keyNum_t = 210;
    pub const K_JOY25: keyNum_t = 209;
    pub const K_JOY24: keyNum_t = 208;
    pub const K_JOY23: keyNum_t = 207;
    pub const K_JOY22: keyNum_t = 206;
    pub const K_JOY21: keyNum_t = 205;
    pub const K_JOY20: keyNum_t = 204;
    pub const K_JOY19: keyNum_t = 203;
    pub const K_JOY18: keyNum_t = 202;
    pub const K_JOY17: keyNum_t = 201;
    pub const K_JOY16: keyNum_t = 200;
    pub const K_JOY15: keyNum_t = 199;
    pub const K_JOY14: keyNum_t = 198;
    pub const K_JOY13: keyNum_t = 197;
    pub const K_JOY12: keyNum_t = 196;
    pub const K_JOY11: keyNum_t = 195;
    pub const K_JOY10: keyNum_t = 194;
    pub const K_JOY9: keyNum_t = 193;
    pub const K_JOY8: keyNum_t = 192;
    pub const K_JOY7: keyNum_t = 191;
    pub const K_JOY6: keyNum_t = 190;
    pub const K_JOY5: keyNum_t = 189;
    pub const K_JOY4: keyNum_t = 188;
    pub const K_JOY3: keyNum_t = 187;
    pub const K_JOY2: keyNum_t = 186;
    pub const K_JOY1: keyNum_t = 185;
    pub const K_KP_EQUALS: keyNum_t = 177;
    pub const K_KP_STAR: keyNum_t = 176;
    pub const K_KP_NUMLOCK: keyNum_t = 175;
    pub const K_KP_PLUS: keyNum_t = 174;
    pub const K_KP_MINUS: keyNum_t = 173;
    pub const K_KP_SLASH: keyNum_t = 172;
    pub const K_KP_DEL: keyNum_t = 171;
    pub const K_KP_INS: keyNum_t = 170;
    pub const K_KP_ENTER: keyNum_t = 169;
    pub const K_KP_PGDN: keyNum_t = 168;
    pub const K_KP_DOWNARROW: keyNum_t = 167;
    pub const K_KP_END: keyNum_t = 166;
    pub const K_KP_RIGHTARROW: keyNum_t = 165;
    pub const K_KP_5: keyNum_t = 164;
    pub const K_KP_LEFTARROW: keyNum_t = 163;
    pub const K_KP_PGUP: keyNum_t = 162;
    pub const K_KP_UPARROW: keyNum_t = 161;
    pub const K_KP_HOME: keyNum_t = 160;
    pub const K_F15: keyNum_t = 159;
    pub const K_F14: keyNum_t = 158;
    pub const K_F13: keyNum_t = 157;
    pub const K_F12: keyNum_t = 156;
    pub const K_F11: keyNum_t = 155;
    pub const K_F10: keyNum_t = 154;
    pub const K_F9: keyNum_t = 153;
    pub const K_F8: keyNum_t = 152;
    pub const K_F7: keyNum_t = 151;
    pub const K_F6: keyNum_t = 150;
    pub const K_F5: keyNum_t = 149;
    pub const K_F4: keyNum_t = 148;
    pub const K_F3: keyNum_t = 147;
    pub const K_F2: keyNum_t = 146;
    pub const K_F1: keyNum_t = 145;
    pub const K_END: keyNum_t = 144;
    pub const K_HOME: keyNum_t = 143;
    pub const K_PGUP: keyNum_t = 142;
    pub const K_PGDN: keyNum_t = 141;
    pub const K_DEL: keyNum_t = 140;
    pub const K_INS: keyNum_t = 139;
    pub const K_SHIFT: keyNum_t = 138;
    pub const K_CTRL: keyNum_t = 137;
    pub const K_ALT: keyNum_t = 136;
    pub const K_RIGHTARROW: keyNum_t = 135;
    pub const K_LEFTARROW: keyNum_t = 134;
    pub const K_DOWNARROW: keyNum_t = 133;
    pub const K_UPARROW: keyNum_t = 132;
    pub const K_PAUSE: keyNum_t = 131;
    pub const K_POWER: keyNum_t = 130;
    pub const K_CAPSLOCK: keyNum_t = 129;
    pub const K_COMMAND: keyNum_t = 128;
    pub const K_BACKSPACE: keyNum_t = 127;
    pub const K_SPACE: keyNum_t = 32;
    pub const K_ESCAPE: keyNum_t = 27;
    pub const K_ENTER: keyNum_t = 13;
    pub const K_TAB: keyNum_t = 9;
    use super::{libc};
}
#[header_src =
      "ioq3/code/client/keys.h"]
pub mod keys_h {
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
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct qkey_t {
        pub down: qboolean,
        pub repeats: libc::c_int,
        pub binding: *mut libc::c_char,
    }
    use super::q_shared_h::{qboolean};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub static mut keys: [qkey_t; 366];
        #[no_mangle]
        pub fn Key_GetBinding(keynum: libc::c_int) -> *mut libc::c_char;
    }
}
#[header_src = "/usr/include/opus/opus.h"]
pub mod opus_h {
    extern "C" {
        /* Copyright (c) 2010-2011 Xiph.Org Foundation, Skype Limited
   Written by Jean-Marc Valin and Koen Vos */
/*
   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions
   are met:

   - Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.

   - Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
   OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
        /* *
 * @file opus.h
 * @brief Opus reference implementation API
 */
        /* *
 * @mainpage Opus
 *
 * The Opus codec is designed for interactive speech and audio transmission over the Internet.
 * It is designed by the IETF Codec Working Group and incorporates technology from
 * Skype's SILK codec and Xiph.Org's CELT codec.
 *
 * The Opus codec is designed to handle a wide range of interactive audio applications,
 * including Voice over IP, videoconferencing, in-game chat, and even remote live music
 * performances. It can scale from low bit-rate narrowband speech to very high quality
 * stereo music. Its main features are:

 * @li Sampling rates from 8 to 48 kHz
 * @li Bit-rates from 6 kb/s to 510 kb/s
 * @li Support for both constant bit-rate (CBR) and variable bit-rate (VBR)
 * @li Audio bandwidth from narrowband to full-band
 * @li Support for speech and music
 * @li Support for mono and stereo
 * @li Support for multichannel (up to 255 channels)
 * @li Frame sizes from 2.5 ms to 60 ms
 * @li Good loss robustness and packet loss concealment (PLC)
 * @li Floating point and fixed-point implementation
 *
 * Documentation sections:
 * @li @ref opus_encoder
 * @li @ref opus_decoder
 * @li @ref opus_repacketizer
 * @li @ref opus_multistream
 * @li @ref opus_libinfo
 * @li @ref opus_custom
 */
        /* * @defgroup opus_encoder Opus Encoder
  * @{
  *
  * @brief This page describes the process and functions used to encode Opus.
  *
  * Since Opus is a stateful codec, the encoding process starts with creating an encoder
  * state. This can be done with:
  *
  * @code
  * int          error;
  * OpusEncoder *enc;
  * enc = opus_encoder_create(Fs, channels, application, &error);
  * @endcode
  *
  * From this point, @c enc can be used for encoding an audio stream. An encoder state
  * @b must @b not be used for more than one stream at the same time. Similarly, the encoder
  * state @b must @b not be re-initialized for each frame.
  *
  * While opus_encoder_create() allocates memory for the state, it's also possible
  * to initialize pre-allocated memory:
  *
  * @code
  * int          size;
  * int          error;
  * OpusEncoder *enc;
  * size = opus_encoder_get_size(channels);
  * enc = malloc(size);
  * error = opus_encoder_init(enc, Fs, channels, application);
  * @endcode
  *
  * where opus_encoder_get_size() returns the required size for the encoder state. Note that
  * future versions of this code may change the size, so no assuptions should be made about it.
  *
  * The encoder state is always continuous in memory and only a shallow copy is sufficient
  * to copy it (e.g. memcpy())
  *
  * It is possible to change some of the encoder's settings using the opus_encoder_ctl()
  * interface. All these settings already default to the recommended value, so they should
  * only be changed when necessary. The most common settings one may want to change are:
  *
  * @code
  * opus_encoder_ctl(enc, OPUS_SET_BITRATE(bitrate));
  * opus_encoder_ctl(enc, OPUS_SET_COMPLEXITY(complexity));
  * opus_encoder_ctl(enc, OPUS_SET_SIGNAL(signal_type));
  * @endcode
  *
  * where
  *
  * @arg bitrate is in bits per second (b/s)
  * @arg complexity is a value from 1 to 10, where 1 is the lowest complexity and 10 is the highest
  * @arg signal_type is either OPUS_AUTO (default), OPUS_SIGNAL_VOICE, or OPUS_SIGNAL_MUSIC
  *
  * See @ref opus_encoderctls and @ref opus_genericctls for a complete list of parameters that can be set or queried. Most parameters can be set or changed at any time during a stream.
  *
  * To encode a frame, opus_encode() or opus_encode_float() must be called with exactly one frame (2.5, 5, 10, 20, 40 or 60 ms) of audio data:
  * @code
  * len = opus_encode(enc, audio_frame, frame_size, packet, max_packet);
  * @endcode
  *
  * where
  * <ul>
  * <li>audio_frame is the audio data in opus_int16 (or float for opus_encode_float())</li>
  * <li>frame_size is the duration of the frame in samples (per channel)</li>
  * <li>packet is the byte array to which the compressed data is written</li>
  * <li>max_packet is the maximum number of bytes that can be written in the packet (4000 bytes is recommended).
  *     Do not use max_packet to control VBR target bitrate, instead use the #OPUS_SET_BITRATE CTL.</li>
  * </ul>
  *
  * opus_encode() and opus_encode_float() return the number of bytes actually written to the packet.
  * The return value <b>can be negative</b>, which indicates that an error has occurred. If the return value
  * is 2 bytes or less, then the packet does not need to be transmitted (DTX).
  *
  * Once the encoder state if no longer needed, it can be destroyed with
  *
  * @code
  * opus_encoder_destroy(enc);
  * @endcode
  *
  * If the encoder was created with opus_encoder_init() rather than opus_encoder_create(),
  * then no action is required aside from potentially freeing the memory that was manually
  * allocated for it (calling free(enc) for the example above)
  *
  */
        /* * Opus encoder state.
  * This contains the complete state of an Opus encoder.
  * It is position independent and can be freely copied.
  * @see opus_encoder_create,opus_encoder_init
  */
        pub type OpusEncoder;
        /* *@}*/
        /* * @defgroup opus_decoder Opus Decoder
  * @{
  *
  * @brief This page describes the process and functions used to decode Opus.
  *
  * The decoding process also starts with creating a decoder
  * state. This can be done with:
  * @code
  * int          error;
  * OpusDecoder *dec;
  * dec = opus_decoder_create(Fs, channels, &error);
  * @endcode
  * where
  * @li Fs is the sampling rate and must be 8000, 12000, 16000, 24000, or 48000
  * @li channels is the number of channels (1 or 2)
  * @li error will hold the error code in case of failure (or #OPUS_OK on success)
  * @li the return value is a newly created decoder state to be used for decoding
  *
  * While opus_decoder_create() allocates memory for the state, it's also possible
  * to initialize pre-allocated memory:
  * @code
  * int          size;
  * int          error;
  * OpusDecoder *dec;
  * size = opus_decoder_get_size(channels);
  * dec = malloc(size);
  * error = opus_decoder_init(dec, Fs, channels);
  * @endcode
  * where opus_decoder_get_size() returns the required size for the decoder state. Note that
  * future versions of this code may change the size, so no assuptions should be made about it.
  *
  * The decoder state is always continuous in memory and only a shallow copy is sufficient
  * to copy it (e.g. memcpy())
  *
  * To decode a frame, opus_decode() or opus_decode_float() must be called with a packet of compressed audio data:
  * @code
  * frame_size = opus_decode(dec, packet, len, decoded, max_size, 0);
  * @endcode
  * where
  *
  * @li packet is the byte array containing the compressed data
  * @li len is the exact number of bytes contained in the packet
  * @li decoded is the decoded audio data in opus_int16 (or float for opus_decode_float())
  * @li max_size is the max duration of the frame in samples (per channel) that can fit into the decoded_frame array
  *
  * opus_decode() and opus_decode_float() return the number of samples (per channel) decoded from the packet.
  * If that value is negative, then an error has occurred. This can occur if the packet is corrupted or if the audio
  * buffer is too small to hold the decoded audio.
  *
  * Opus is a stateful codec with overlapping blocks and as a result Opus
  * packets are not coded independently of each other. Packets must be
  * passed into the decoder serially and in the correct order for a correct
  * decode. Lost packets can be replaced with loss concealment by calling
  * the decoder with a null pointer and zero length for the missing packet.
  *
  * A single codec state may only be accessed from a single thread at
  * a time and any required locking must be performed by the caller. Separate
  * streams must be decoded with separate decoder states and can be decoded
  * in parallel unless the library was compiled with NONTHREADSAFE_PSEUDOSTACK
  * defined.
  *
  */
        /* * Opus decoder state.
  * This contains the complete state of an Opus decoder.
  * It is position independent and can be freely copied.
  * @see opus_decoder_create,opus_decoder_init
  */
        pub type OpusDecoder;
    }
}
#[header_src = "/usr/include/curl/multi.h"]
pub mod multi_h {
    pub type CURLM = ();
    use super::{libc};
}
#[header_src = "/usr/include/curl/curl.h"]
pub mod curl_h {
    pub type CURL = ();
    use super::{libc};
}
#[header_src = "/usr/include/stdlib.h"]
pub mod stdlib_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn abs(_: libc::c_int) -> libc::c_int;
    }
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[header_src = "/usr/include/SDL2/SDL_error.h"]
pub mod SDL_error_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn SDL_GetError() -> *const libc::c_char;
    }
}
#[header_src = "/usr/include/SDL2/SDL_mouse.h"]
pub mod SDL_mouse_h {
    use super::SDL_video_h::{SDL_Window};
    use super::{libc};
    use super::SDL_stdinc_h::{SDL_bool};
    extern "C" {
        /* *
 *  \brief Moves the mouse to the given position within the window.
 *
 *  \param window The window to move the mouse into, or NULL for the current mouse focus
 *  \param x The x coordinate within the window
 *  \param y The y coordinate within the window
 *
 *  \note This function generates a mouse motion event
 */
        #[no_mangle]
        pub fn SDL_WarpMouseInWindow(window: *mut SDL_Window, x: libc::c_int,
                                     y: libc::c_int);
        /* *
 *  \brief Set relative mouse mode.
 *
 *  \param enabled Whether or not to enable relative mode
 *
 *  \return 0 on success, or -1 if relative mode is not supported.
 *
 *  While the mouse is in relative mode, the cursor is hidden, and the
 *  driver will try to report continuous motion in the current window.
 *  Only relative motion events will be delivered, the mouse position
 *  will not change.
 *
 *  \note This function will flush any pending mouse motion.
 *
 *  \sa SDL_GetRelativeMouseMode()
 */
        #[no_mangle]
        pub fn SDL_SetRelativeMouseMode(enabled: SDL_bool) -> libc::c_int;
        /* *
 *  \brief Toggle whether or not the cursor is shown.
 *
 *  \param toggle 1 to show the cursor, 0 to hide it, -1 to query the current
 *                state.
 *
 *  \return 1 if the cursor is shown, or 0 if the cursor is hidden.
 */
        #[no_mangle]
        pub fn SDL_ShowCursor(toggle: libc::c_int) -> libc::c_int;
    }
}
#[header_src = "/usr/include/SDL2/SDL.h"]
pub mod SDL_h {
    use super::{libc};
    use super::SDL_stdinc_h::{Uint32};
    extern "C" {
        /*
  Simple DirectMedia Layer
  Copyright (C) 1997-2018 Sam Lantinga <slouken@libsdl.org>

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.
*/
        /* *
 *  \file SDL.h
 *
 *  Main include header for the SDL library
 */
        /* Set up for C function definitions, even when using C++ */
        /* As of version 0.5, SDL is loaded dynamically into the application */
        /* *
 *  \name SDL_INIT_*
 *
 *  These are the flags which may be passed to SDL_Init().  You should
 *  specify the subsystems which you will be using in your application.
 */
/* @{ */
        /* *< SDL_INIT_VIDEO implies SDL_INIT_EVENTS */
        /* *< SDL_INIT_JOYSTICK implies SDL_INIT_EVENTS */
        /* *< SDL_INIT_GAMECONTROLLER implies SDL_INIT_JOYSTICK */
        /* *< compatibility; this flag is ignored. */
        /* @} */
        /* *
 *  This function initializes  the subsystems specified by \c flags
 */
        #[no_mangle]
        pub fn SDL_Init(flags: Uint32) -> libc::c_int;
        /* *
 *  This function cleans up specific SDL subsystems
 */
        #[no_mangle]
        pub fn SDL_QuitSubSystem(flags: Uint32);
        /* *
 *  This function returns a mask of the specified subsystems which have
 *  previously been initialized.
 *
 *  If \c flags is 0, it returns a mask of all initialized subsystems.
 */
        #[no_mangle]
        pub fn SDL_WasInit(flags: Uint32) -> Uint32;
    }
}
use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t, __uint32_t,
                    __int64_t};
use self::stdint_intn_h::{int16_t, int32_t, int64_t};
use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
use self::SDL_stdinc_h::{SDL_bool, SDL_TRUE, SDL_FALSE, Uint8, Sint16, Uint16,
                         Sint32, Uint32, Sint64};
use self::SDL_video_h::{unnamed, SDL_WINDOW_VULKAN, SDL_WINDOW_POPUP_MENU,
                        SDL_WINDOW_TOOLTIP, SDL_WINDOW_UTILITY,
                        SDL_WINDOW_SKIP_TASKBAR, SDL_WINDOW_ALWAYS_ON_TOP,
                        SDL_WINDOW_MOUSE_CAPTURE, SDL_WINDOW_ALLOW_HIGHDPI,
                        SDL_WINDOW_FOREIGN, SDL_WINDOW_FULLSCREEN_DESKTOP,
                        SDL_WINDOW_MOUSE_FOCUS, SDL_WINDOW_INPUT_FOCUS,
                        SDL_WINDOW_INPUT_GRABBED, SDL_WINDOW_MAXIMIZED,
                        SDL_WINDOW_MINIMIZED, SDL_WINDOW_RESIZABLE,
                        SDL_WINDOW_BORDERLESS, SDL_WINDOW_HIDDEN,
                        SDL_WINDOW_SHOWN, SDL_WINDOW_OPENGL,
                        SDL_WINDOW_FULLSCREEN, unnamed_0,
                        SDL_WINDOWEVENT_HIT_TEST, SDL_WINDOWEVENT_TAKE_FOCUS,
                        SDL_WINDOWEVENT_CLOSE, SDL_WINDOWEVENT_FOCUS_LOST,
                        SDL_WINDOWEVENT_FOCUS_GAINED, SDL_WINDOWEVENT_LEAVE,
                        SDL_WINDOWEVENT_ENTER, SDL_WINDOWEVENT_RESTORED,
                        SDL_WINDOWEVENT_MAXIMIZED, SDL_WINDOWEVENT_MINIMIZED,
                        SDL_WINDOWEVENT_SIZE_CHANGED, SDL_WINDOWEVENT_RESIZED,
                        SDL_WINDOWEVENT_MOVED, SDL_WINDOWEVENT_EXPOSED,
                        SDL_WINDOWEVENT_HIDDEN, SDL_WINDOWEVENT_SHOWN,
                        SDL_WINDOWEVENT_NONE, SDL_Window, SDL_GetWindowFlags,
                        SDL_SetWindowGrab};
use self::SDL_scancode_h::{SDL_Scancode, SDL_NUM_SCANCODES,
                           SDL_SCANCODE_AUDIOFASTFORWARD,
                           SDL_SCANCODE_AUDIOREWIND, SDL_SCANCODE_APP2,
                           SDL_SCANCODE_APP1, SDL_SCANCODE_SLEEP,
                           SDL_SCANCODE_EJECT, SDL_SCANCODE_KBDILLUMUP,
                           SDL_SCANCODE_KBDILLUMDOWN,
                           SDL_SCANCODE_KBDILLUMTOGGLE,
                           SDL_SCANCODE_DISPLAYSWITCH,
                           SDL_SCANCODE_BRIGHTNESSUP,
                           SDL_SCANCODE_BRIGHTNESSDOWN,
                           SDL_SCANCODE_AC_BOOKMARKS, SDL_SCANCODE_AC_REFRESH,
                           SDL_SCANCODE_AC_STOP, SDL_SCANCODE_AC_FORWARD,
                           SDL_SCANCODE_AC_BACK, SDL_SCANCODE_AC_HOME,
                           SDL_SCANCODE_AC_SEARCH, SDL_SCANCODE_COMPUTER,
                           SDL_SCANCODE_CALCULATOR, SDL_SCANCODE_MAIL,
                           SDL_SCANCODE_WWW, SDL_SCANCODE_MEDIASELECT,
                           SDL_SCANCODE_AUDIOMUTE, SDL_SCANCODE_AUDIOPLAY,
                           SDL_SCANCODE_AUDIOSTOP, SDL_SCANCODE_AUDIOPREV,
                           SDL_SCANCODE_AUDIONEXT, SDL_SCANCODE_MODE,
                           SDL_SCANCODE_RGUI, SDL_SCANCODE_RALT,
                           SDL_SCANCODE_RSHIFT, SDL_SCANCODE_RCTRL,
                           SDL_SCANCODE_LGUI, SDL_SCANCODE_LALT,
                           SDL_SCANCODE_LSHIFT, SDL_SCANCODE_LCTRL,
                           SDL_SCANCODE_KP_HEXADECIMAL,
                           SDL_SCANCODE_KP_DECIMAL, SDL_SCANCODE_KP_OCTAL,
                           SDL_SCANCODE_KP_BINARY, SDL_SCANCODE_KP_CLEARENTRY,
                           SDL_SCANCODE_KP_CLEAR, SDL_SCANCODE_KP_PLUSMINUS,
                           SDL_SCANCODE_KP_MEMDIVIDE,
                           SDL_SCANCODE_KP_MEMMULTIPLY,
                           SDL_SCANCODE_KP_MEMSUBTRACT,
                           SDL_SCANCODE_KP_MEMADD, SDL_SCANCODE_KP_MEMCLEAR,
                           SDL_SCANCODE_KP_MEMRECALL,
                           SDL_SCANCODE_KP_MEMSTORE, SDL_SCANCODE_KP_EXCLAM,
                           SDL_SCANCODE_KP_AT, SDL_SCANCODE_KP_SPACE,
                           SDL_SCANCODE_KP_HASH, SDL_SCANCODE_KP_COLON,
                           SDL_SCANCODE_KP_DBLVERTICALBAR,
                           SDL_SCANCODE_KP_VERTICALBAR,
                           SDL_SCANCODE_KP_DBLAMPERSAND,
                           SDL_SCANCODE_KP_AMPERSAND, SDL_SCANCODE_KP_GREATER,
                           SDL_SCANCODE_KP_LESS, SDL_SCANCODE_KP_PERCENT,
                           SDL_SCANCODE_KP_POWER, SDL_SCANCODE_KP_XOR,
                           SDL_SCANCODE_KP_F, SDL_SCANCODE_KP_E,
                           SDL_SCANCODE_KP_D, SDL_SCANCODE_KP_C,
                           SDL_SCANCODE_KP_B, SDL_SCANCODE_KP_A,
                           SDL_SCANCODE_KP_BACKSPACE, SDL_SCANCODE_KP_TAB,
                           SDL_SCANCODE_KP_RIGHTBRACE,
                           SDL_SCANCODE_KP_LEFTBRACE,
                           SDL_SCANCODE_KP_RIGHTPAREN,
                           SDL_SCANCODE_KP_LEFTPAREN,
                           SDL_SCANCODE_CURRENCYSUBUNIT,
                           SDL_SCANCODE_CURRENCYUNIT,
                           SDL_SCANCODE_DECIMALSEPARATOR,
                           SDL_SCANCODE_THOUSANDSSEPARATOR,
                           SDL_SCANCODE_KP_000, SDL_SCANCODE_KP_00,
                           SDL_SCANCODE_EXSEL, SDL_SCANCODE_CRSEL,
                           SDL_SCANCODE_CLEARAGAIN, SDL_SCANCODE_OPER,
                           SDL_SCANCODE_OUT, SDL_SCANCODE_SEPARATOR,
                           SDL_SCANCODE_RETURN2, SDL_SCANCODE_PRIOR,
                           SDL_SCANCODE_CLEAR, SDL_SCANCODE_CANCEL,
                           SDL_SCANCODE_SYSREQ, SDL_SCANCODE_ALTERASE,
                           SDL_SCANCODE_LANG9, SDL_SCANCODE_LANG8,
                           SDL_SCANCODE_LANG7, SDL_SCANCODE_LANG6,
                           SDL_SCANCODE_LANG5, SDL_SCANCODE_LANG4,
                           SDL_SCANCODE_LANG3, SDL_SCANCODE_LANG2,
                           SDL_SCANCODE_LANG1, SDL_SCANCODE_INTERNATIONAL9,
                           SDL_SCANCODE_INTERNATIONAL8,
                           SDL_SCANCODE_INTERNATIONAL7,
                           SDL_SCANCODE_INTERNATIONAL6,
                           SDL_SCANCODE_INTERNATIONAL5,
                           SDL_SCANCODE_INTERNATIONAL4,
                           SDL_SCANCODE_INTERNATIONAL3,
                           SDL_SCANCODE_INTERNATIONAL2,
                           SDL_SCANCODE_INTERNATIONAL1,
                           SDL_SCANCODE_KP_EQUALSAS400, SDL_SCANCODE_KP_COMMA,
                           SDL_SCANCODE_VOLUMEDOWN, SDL_SCANCODE_VOLUMEUP,
                           SDL_SCANCODE_MUTE, SDL_SCANCODE_FIND,
                           SDL_SCANCODE_PASTE, SDL_SCANCODE_COPY,
                           SDL_SCANCODE_CUT, SDL_SCANCODE_UNDO,
                           SDL_SCANCODE_AGAIN, SDL_SCANCODE_STOP,
                           SDL_SCANCODE_SELECT, SDL_SCANCODE_MENU,
                           SDL_SCANCODE_HELP, SDL_SCANCODE_EXECUTE,
                           SDL_SCANCODE_F24, SDL_SCANCODE_F23,
                           SDL_SCANCODE_F22, SDL_SCANCODE_F21,
                           SDL_SCANCODE_F20, SDL_SCANCODE_F19,
                           SDL_SCANCODE_F18, SDL_SCANCODE_F17,
                           SDL_SCANCODE_F16, SDL_SCANCODE_F15,
                           SDL_SCANCODE_F14, SDL_SCANCODE_F13,
                           SDL_SCANCODE_KP_EQUALS, SDL_SCANCODE_POWER,
                           SDL_SCANCODE_APPLICATION,
                           SDL_SCANCODE_NONUSBACKSLASH,
                           SDL_SCANCODE_KP_PERIOD, SDL_SCANCODE_KP_0,
                           SDL_SCANCODE_KP_9, SDL_SCANCODE_KP_8,
                           SDL_SCANCODE_KP_7, SDL_SCANCODE_KP_6,
                           SDL_SCANCODE_KP_5, SDL_SCANCODE_KP_4,
                           SDL_SCANCODE_KP_3, SDL_SCANCODE_KP_2,
                           SDL_SCANCODE_KP_1, SDL_SCANCODE_KP_ENTER,
                           SDL_SCANCODE_KP_PLUS, SDL_SCANCODE_KP_MINUS,
                           SDL_SCANCODE_KP_MULTIPLY, SDL_SCANCODE_KP_DIVIDE,
                           SDL_SCANCODE_NUMLOCKCLEAR, SDL_SCANCODE_UP,
                           SDL_SCANCODE_DOWN, SDL_SCANCODE_LEFT,
                           SDL_SCANCODE_RIGHT, SDL_SCANCODE_PAGEDOWN,
                           SDL_SCANCODE_END, SDL_SCANCODE_DELETE,
                           SDL_SCANCODE_PAGEUP, SDL_SCANCODE_HOME,
                           SDL_SCANCODE_INSERT, SDL_SCANCODE_PAUSE,
                           SDL_SCANCODE_SCROLLLOCK, SDL_SCANCODE_PRINTSCREEN,
                           SDL_SCANCODE_F12, SDL_SCANCODE_F11,
                           SDL_SCANCODE_F10, SDL_SCANCODE_F9, SDL_SCANCODE_F8,
                           SDL_SCANCODE_F7, SDL_SCANCODE_F6, SDL_SCANCODE_F5,
                           SDL_SCANCODE_F4, SDL_SCANCODE_F3, SDL_SCANCODE_F2,
                           SDL_SCANCODE_F1, SDL_SCANCODE_CAPSLOCK,
                           SDL_SCANCODE_SLASH, SDL_SCANCODE_PERIOD,
                           SDL_SCANCODE_COMMA, SDL_SCANCODE_GRAVE,
                           SDL_SCANCODE_APOSTROPHE, SDL_SCANCODE_SEMICOLON,
                           SDL_SCANCODE_NONUSHASH, SDL_SCANCODE_BACKSLASH,
                           SDL_SCANCODE_RIGHTBRACKET,
                           SDL_SCANCODE_LEFTBRACKET, SDL_SCANCODE_EQUALS,
                           SDL_SCANCODE_MINUS, SDL_SCANCODE_SPACE,
                           SDL_SCANCODE_TAB, SDL_SCANCODE_BACKSPACE,
                           SDL_SCANCODE_ESCAPE, SDL_SCANCODE_RETURN,
                           SDL_SCANCODE_0, SDL_SCANCODE_9, SDL_SCANCODE_8,
                           SDL_SCANCODE_7, SDL_SCANCODE_6, SDL_SCANCODE_5,
                           SDL_SCANCODE_4, SDL_SCANCODE_3, SDL_SCANCODE_2,
                           SDL_SCANCODE_1, SDL_SCANCODE_Z, SDL_SCANCODE_Y,
                           SDL_SCANCODE_X, SDL_SCANCODE_W, SDL_SCANCODE_V,
                           SDL_SCANCODE_U, SDL_SCANCODE_T, SDL_SCANCODE_S,
                           SDL_SCANCODE_R, SDL_SCANCODE_Q, SDL_SCANCODE_P,
                           SDL_SCANCODE_O, SDL_SCANCODE_N, SDL_SCANCODE_M,
                           SDL_SCANCODE_L, SDL_SCANCODE_K, SDL_SCANCODE_J,
                           SDL_SCANCODE_I, SDL_SCANCODE_H, SDL_SCANCODE_G,
                           SDL_SCANCODE_F, SDL_SCANCODE_E, SDL_SCANCODE_D,
                           SDL_SCANCODE_C, SDL_SCANCODE_B, SDL_SCANCODE_A,
                           SDL_SCANCODE_UNKNOWN};
use self::SDL_keycode_h::{SDL_Keycode, unnamed_1, SDLK_AUDIOFASTFORWARD,
                          SDLK_AUDIOREWIND, SDLK_APP2, SDLK_APP1, SDLK_SLEEP,
                          SDLK_EJECT, SDLK_KBDILLUMUP, SDLK_KBDILLUMDOWN,
                          SDLK_KBDILLUMTOGGLE, SDLK_DISPLAYSWITCH,
                          SDLK_BRIGHTNESSUP, SDLK_BRIGHTNESSDOWN,
                          SDLK_AC_BOOKMARKS, SDLK_AC_REFRESH, SDLK_AC_STOP,
                          SDLK_AC_FORWARD, SDLK_AC_BACK, SDLK_AC_HOME,
                          SDLK_AC_SEARCH, SDLK_COMPUTER, SDLK_CALCULATOR,
                          SDLK_MAIL, SDLK_WWW, SDLK_MEDIASELECT,
                          SDLK_AUDIOMUTE, SDLK_AUDIOPLAY, SDLK_AUDIOSTOP,
                          SDLK_AUDIOPREV, SDLK_AUDIONEXT, SDLK_MODE,
                          SDLK_RGUI, SDLK_RALT, SDLK_RSHIFT, SDLK_RCTRL,
                          SDLK_LGUI, SDLK_LALT, SDLK_LSHIFT, SDLK_LCTRL,
                          SDLK_KP_HEXADECIMAL, SDLK_KP_DECIMAL, SDLK_KP_OCTAL,
                          SDLK_KP_BINARY, SDLK_KP_CLEARENTRY, SDLK_KP_CLEAR,
                          SDLK_KP_PLUSMINUS, SDLK_KP_MEMDIVIDE,
                          SDLK_KP_MEMMULTIPLY, SDLK_KP_MEMSUBTRACT,
                          SDLK_KP_MEMADD, SDLK_KP_MEMCLEAR, SDLK_KP_MEMRECALL,
                          SDLK_KP_MEMSTORE, SDLK_KP_EXCLAM, SDLK_KP_AT,
                          SDLK_KP_SPACE, SDLK_KP_HASH, SDLK_KP_COLON,
                          SDLK_KP_DBLVERTICALBAR, SDLK_KP_VERTICALBAR,
                          SDLK_KP_DBLAMPERSAND, SDLK_KP_AMPERSAND,
                          SDLK_KP_GREATER, SDLK_KP_LESS, SDLK_KP_PERCENT,
                          SDLK_KP_POWER, SDLK_KP_XOR, SDLK_KP_F, SDLK_KP_E,
                          SDLK_KP_D, SDLK_KP_C, SDLK_KP_B, SDLK_KP_A,
                          SDLK_KP_BACKSPACE, SDLK_KP_TAB, SDLK_KP_RIGHTBRACE,
                          SDLK_KP_LEFTBRACE, SDLK_KP_RIGHTPAREN,
                          SDLK_KP_LEFTPAREN, SDLK_CURRENCYSUBUNIT,
                          SDLK_CURRENCYUNIT, SDLK_DECIMALSEPARATOR,
                          SDLK_THOUSANDSSEPARATOR, SDLK_KP_000, SDLK_KP_00,
                          SDLK_EXSEL, SDLK_CRSEL, SDLK_CLEARAGAIN, SDLK_OPER,
                          SDLK_OUT, SDLK_SEPARATOR, SDLK_RETURN2, SDLK_PRIOR,
                          SDLK_CLEAR, SDLK_CANCEL, SDLK_SYSREQ, SDLK_ALTERASE,
                          SDLK_KP_EQUALSAS400, SDLK_KP_COMMA, SDLK_VOLUMEDOWN,
                          SDLK_VOLUMEUP, SDLK_MUTE, SDLK_FIND, SDLK_PASTE,
                          SDLK_COPY, SDLK_CUT, SDLK_UNDO, SDLK_AGAIN,
                          SDLK_STOP, SDLK_SELECT, SDLK_MENU, SDLK_HELP,
                          SDLK_EXECUTE, SDLK_F24, SDLK_F23, SDLK_F22,
                          SDLK_F21, SDLK_F20, SDLK_F19, SDLK_F18, SDLK_F17,
                          SDLK_F16, SDLK_F15, SDLK_F14, SDLK_F13,
                          SDLK_KP_EQUALS, SDLK_POWER, SDLK_APPLICATION,
                          SDLK_KP_PERIOD, SDLK_KP_0, SDLK_KP_9, SDLK_KP_8,
                          SDLK_KP_7, SDLK_KP_6, SDLK_KP_5, SDLK_KP_4,
                          SDLK_KP_3, SDLK_KP_2, SDLK_KP_1, SDLK_KP_ENTER,
                          SDLK_KP_PLUS, SDLK_KP_MINUS, SDLK_KP_MULTIPLY,
                          SDLK_KP_DIVIDE, SDLK_NUMLOCKCLEAR, SDLK_UP,
                          SDLK_DOWN, SDLK_LEFT, SDLK_RIGHT, SDLK_PAGEDOWN,
                          SDLK_END, SDLK_DELETE, SDLK_PAGEUP, SDLK_HOME,
                          SDLK_INSERT, SDLK_PAUSE, SDLK_SCROLLLOCK,
                          SDLK_PRINTSCREEN, SDLK_F12, SDLK_F11, SDLK_F10,
                          SDLK_F9, SDLK_F8, SDLK_F7, SDLK_F6, SDLK_F5,
                          SDLK_F4, SDLK_F3, SDLK_F2, SDLK_F1, SDLK_CAPSLOCK,
                          SDLK_z, SDLK_y, SDLK_x, SDLK_w, SDLK_v, SDLK_u,
                          SDLK_t, SDLK_s, SDLK_r, SDLK_q, SDLK_p, SDLK_o,
                          SDLK_n, SDLK_m, SDLK_l, SDLK_k, SDLK_j, SDLK_i,
                          SDLK_h, SDLK_g, SDLK_f, SDLK_e, SDLK_d, SDLK_c,
                          SDLK_b, SDLK_a, SDLK_BACKQUOTE, SDLK_UNDERSCORE,
                          SDLK_CARET, SDLK_RIGHTBRACKET, SDLK_BACKSLASH,
                          SDLK_LEFTBRACKET, SDLK_AT, SDLK_QUESTION,
                          SDLK_GREATER, SDLK_EQUALS, SDLK_LESS,
                          SDLK_SEMICOLON, SDLK_COLON, SDLK_9, SDLK_8, SDLK_7,
                          SDLK_6, SDLK_5, SDLK_4, SDLK_3, SDLK_2, SDLK_1,
                          SDLK_0, SDLK_SLASH, SDLK_PERIOD, SDLK_MINUS,
                          SDLK_COMMA, SDLK_PLUS, SDLK_ASTERISK,
                          SDLK_RIGHTPAREN, SDLK_LEFTPAREN, SDLK_QUOTE,
                          SDLK_AMPERSAND, SDLK_DOLLAR, SDLK_PERCENT,
                          SDLK_HASH, SDLK_QUOTEDBL, SDLK_EXCLAIM, SDLK_SPACE,
                          SDLK_TAB, SDLK_BACKSPACE, SDLK_ESCAPE, SDLK_RETURN,
                          SDLK_UNKNOWN, unnamed_2, KMOD_RESERVED, KMOD_MODE,
                          KMOD_CAPS, KMOD_NUM, KMOD_RGUI, KMOD_LGUI,
                          KMOD_RALT, KMOD_LALT, KMOD_RCTRL, KMOD_LCTRL,
                          KMOD_RSHIFT, KMOD_LSHIFT, KMOD_NONE};
use self::SDL_keyboard_h::{SDL_Keysym, SDL_GetScancodeName, SDL_GetKeyName,
                           SDL_StartTextInput, SDL_StopTextInput};
use self::SDL_joystick_h::{SDL_Joystick, SDL_JoystickID, _SDL_Joystick,
                           SDL_NumJoysticks, SDL_JoystickNameForIndex,
                           SDL_JoystickOpen, SDL_JoystickNumAxes,
                           SDL_JoystickNumBalls, SDL_JoystickNumHats,
                           SDL_JoystickNumButtons, SDL_JoystickUpdate,
                           SDL_JoystickEventState, SDL_JoystickGetAxis,
                           SDL_JoystickGetHat, SDL_JoystickGetBall,
                           SDL_JoystickGetButton, SDL_JoystickClose};
use self::SDL_gamecontroller_h::{SDL_GameController, SDL_GameControllerAxis,
                                 SDL_CONTROLLER_AXIS_MAX,
                                 SDL_CONTROLLER_AXIS_TRIGGERRIGHT,
                                 SDL_CONTROLLER_AXIS_TRIGGERLEFT,
                                 SDL_CONTROLLER_AXIS_RIGHTY,
                                 SDL_CONTROLLER_AXIS_RIGHTX,
                                 SDL_CONTROLLER_AXIS_LEFTY,
                                 SDL_CONTROLLER_AXIS_LEFTX,
                                 SDL_CONTROLLER_AXIS_INVALID,
                                 SDL_GameControllerButton,
                                 SDL_CONTROLLER_BUTTON_MAX,
                                 SDL_CONTROLLER_BUTTON_DPAD_RIGHT,
                                 SDL_CONTROLLER_BUTTON_DPAD_LEFT,
                                 SDL_CONTROLLER_BUTTON_DPAD_DOWN,
                                 SDL_CONTROLLER_BUTTON_DPAD_UP,
                                 SDL_CONTROLLER_BUTTON_RIGHTSHOULDER,
                                 SDL_CONTROLLER_BUTTON_LEFTSHOULDER,
                                 SDL_CONTROLLER_BUTTON_RIGHTSTICK,
                                 SDL_CONTROLLER_BUTTON_LEFTSTICK,
                                 SDL_CONTROLLER_BUTTON_START,
                                 SDL_CONTROLLER_BUTTON_GUIDE,
                                 SDL_CONTROLLER_BUTTON_BACK,
                                 SDL_CONTROLLER_BUTTON_Y,
                                 SDL_CONTROLLER_BUTTON_X,
                                 SDL_CONTROLLER_BUTTON_B,
                                 SDL_CONTROLLER_BUTTON_A,
                                 SDL_CONTROLLER_BUTTON_INVALID,
                                 _SDL_GameController, SDL_IsGameController,
                                 SDL_GameControllerOpen,
                                 SDL_GameControllerEventState,
                                 SDL_GameControllerUpdate,
                                 SDL_GameControllerGetAxis,
                                 SDL_GameControllerGetButton,
                                 SDL_GameControllerClose};
use self::SDL_touch_h::{SDL_TouchID, SDL_FingerID};
use self::SDL_gesture_h::{SDL_GestureID};
use self::SDL_events_h::{unnamed_3, SDL_LASTEVENT, SDL_USEREVENT,
                         SDL_RENDER_DEVICE_RESET, SDL_RENDER_TARGETS_RESET,
                         SDL_SENSORUPDATE, SDL_AUDIODEVICEREMOVED,
                         SDL_AUDIODEVICEADDED, SDL_DROPCOMPLETE,
                         SDL_DROPBEGIN, SDL_DROPTEXT, SDL_DROPFILE,
                         SDL_CLIPBOARDUPDATE, SDL_MULTIGESTURE,
                         SDL_DOLLARRECORD, SDL_DOLLARGESTURE,
                         SDL_FINGERMOTION, SDL_FINGERUP, SDL_FINGERDOWN,
                         SDL_CONTROLLERDEVICEREMAPPED,
                         SDL_CONTROLLERDEVICEREMOVED,
                         SDL_CONTROLLERDEVICEADDED, SDL_CONTROLLERBUTTONUP,
                         SDL_CONTROLLERBUTTONDOWN, SDL_CONTROLLERAXISMOTION,
                         SDL_JOYDEVICEREMOVED, SDL_JOYDEVICEADDED,
                         SDL_JOYBUTTONUP, SDL_JOYBUTTONDOWN, SDL_JOYHATMOTION,
                         SDL_JOYBALLMOTION, SDL_JOYAXISMOTION, SDL_MOUSEWHEEL,
                         SDL_MOUSEBUTTONUP, SDL_MOUSEBUTTONDOWN,
                         SDL_MOUSEMOTION, SDL_KEYMAPCHANGED, SDL_TEXTINPUT,
                         SDL_TEXTEDITING, SDL_KEYUP, SDL_KEYDOWN,
                         SDL_SYSWMEVENT, SDL_WINDOWEVENT, SDL_DISPLAYEVENT,
                         SDL_APP_DIDENTERFOREGROUND,
                         SDL_APP_WILLENTERFOREGROUND,
                         SDL_APP_DIDENTERBACKGROUND,
                         SDL_APP_WILLENTERBACKGROUND, SDL_APP_LOWMEMORY,
                         SDL_APP_TERMINATING, SDL_QUIT, SDL_FIRSTEVENT,
                         SDL_CommonEvent, SDL_DisplayEvent, SDL_WindowEvent,
                         SDL_KeyboardEvent, SDL_TextEditingEvent,
                         SDL_TextInputEvent, SDL_MouseMotionEvent,
                         SDL_MouseButtonEvent, SDL_MouseWheelEvent,
                         SDL_JoyAxisEvent, SDL_JoyBallEvent, SDL_JoyHatEvent,
                         SDL_JoyButtonEvent, SDL_JoyDeviceEvent,
                         SDL_ControllerAxisEvent, SDL_ControllerButtonEvent,
                         SDL_ControllerDeviceEvent, SDL_AudioDeviceEvent,
                         SDL_TouchFingerEvent, SDL_MultiGestureEvent,
                         SDL_DollarGestureEvent, SDL_DropEvent,
                         SDL_SensorEvent, SDL_QuitEvent, SDL_UserEvent,
                         SDL_SysWMEvent, SDL_Event, SDL_eventaction,
                         SDL_GETEVENT, SDL_PEEKEVENT, SDL_ADDEVENT,
                         SDL_SysWMmsg, SDL_PumpEvents, SDL_PeepEvents,
                         SDL_PollEvent};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, qhandle_t, fileHandle_t,
                       unnamed_4, EXEC_APPEND, EXEC_INSERT, EXEC_NOW,
                       unnamed_5, ERR_NEED_CD, ERR_DISCONNECT,
                       ERR_SERVERDISCONNECT, ERR_DROP, ERR_FATAL, cvar_s,
                       cvar_t, connstate_t, CA_CINEMATIC, CA_ACTIVE,
                       CA_PRIMED, CA_LOADING, CA_CONNECTED, CA_CHALLENGING,
                       CA_CONNECTING, CA_AUTHORIZING, CA_DISCONNECTED,
                       CA_UNINITIALIZED, COM_Parse, Com_HexStrToInt,
                       Q_stricmp, Q_strcat, Com_Error, Com_Printf};
use self::qcommon_h::{netadrtype_t, NA_UNSPEC, NA_MULTICAST6, NA_IP6, NA_IP,
                      NA_BROADCAST, NA_LOOPBACK, NA_BOT, NA_BAD, netsrc_t,
                      NS_SERVER, NS_CLIENT, netadr_t, netchan_t,
                      sysEventType_t, SE_CONSOLE, SE_JOYSTICK_AXIS, SE_MOUSE,
                      SE_CHAR, SE_KEY, SE_NONE, Cbuf_AddText,
                      Cbuf_ExecuteText, Cvar_Get, Cvar_Set, Cvar_SetValue,
                      Cvar_VariableIntegerValue, Com_QueueEvent, Com_DPrintf,
                      Sys_Milliseconds};
use self::sdl_input_c::{unnamed_6, unnamed_7, consoleKey_t, consoleKey_s,
                        unnamed_8, CHARACTER, QUAKE_KEY};
use self::tr_types_h::{glconfig_t, textureCompression_t, TC_S3TC_ARB, TC_S3TC,
                       TC_NONE, glHardwareType_t, GLHW_PERMEDIA2,
                       GLHW_RAGEPRO, GLHW_RIVA128, GLHW_3DFX_2D3D,
                       GLHW_GENERIC, glDriverType_t, GLDRV_VOODOO,
                       GLDRV_STANDALONE, GLDRV_ICD};
use self::client_h::{clientStatic_t, serverInfo_t, clientConnection_t, cls,
                     Key_StringToKeynum, cl_consoleKeys, Key_KeynumToString,
                     Key_GetCatcher, clc, j_up, j_up_axis, j_yaw, j_yaw_axis,
                     j_pitch, j_pitch_axis, j_side, j_side_axis, j_forward,
                     j_forward_axis};
use self::keycodes_h::{K_MWHEELDOWN, K_MWHEELUP, K_AUX1, K_MOUSE5, K_MOUSE4,
                       K_MOUSE2, K_MOUSE3, K_MOUSE1, K_CONSOLE, keyNum_t,
                       MAX_KEYS, K_PAD0_RIGHTTRIGGER, K_PAD0_LEFTTRIGGER,
                       K_PAD0_RIGHTSTICK_DOWN, K_PAD0_RIGHTSTICK_UP,
                       K_PAD0_RIGHTSTICK_RIGHT, K_PAD0_RIGHTSTICK_LEFT,
                       K_PAD0_LEFTSTICK_DOWN, K_PAD0_LEFTSTICK_UP,
                       K_PAD0_LEFTSTICK_RIGHT, K_PAD0_LEFTSTICK_LEFT,
                       K_PAD0_DPAD_RIGHT, K_PAD0_DPAD_LEFT, K_PAD0_DPAD_DOWN,
                       K_PAD0_DPAD_UP, K_PAD0_RIGHTSHOULDER,
                       K_PAD0_LEFTSHOULDER, K_PAD0_RIGHTSTICK_CLICK,
                       K_PAD0_LEFTSTICK_CLICK, K_PAD0_START, K_PAD0_GUIDE,
                       K_PAD0_BACK, K_PAD0_Y, K_PAD0_X, K_PAD0_B, K_PAD0_A,
                       K_UNDO, K_EURO, K_MENU, K_BREAK, K_SCROLLOCK, K_SYSREQ,
                       K_PRINT, K_HELP, K_MODE, K_COMPOSE, K_SUPER,
                       K_WORLD_95, K_WORLD_94, K_WORLD_93, K_WORLD_92,
                       K_WORLD_91, K_WORLD_90, K_WORLD_89, K_WORLD_88,
                       K_WORLD_87, K_WORLD_86, K_WORLD_85, K_WORLD_84,
                       K_WORLD_83, K_WORLD_82, K_WORLD_81, K_WORLD_80,
                       K_WORLD_79, K_WORLD_78, K_WORLD_77, K_WORLD_76,
                       K_WORLD_75, K_WORLD_74, K_WORLD_73, K_WORLD_72,
                       K_WORLD_71, K_WORLD_70, K_WORLD_69, K_WORLD_68,
                       K_WORLD_67, K_WORLD_66, K_WORLD_65, K_WORLD_64,
                       K_WORLD_63, K_WORLD_62, K_WORLD_61, K_WORLD_60,
                       K_WORLD_59, K_WORLD_58, K_WORLD_57, K_WORLD_56,
                       K_WORLD_55, K_WORLD_54, K_WORLD_53, K_WORLD_52,
                       K_WORLD_51, K_WORLD_50, K_WORLD_49, K_WORLD_48,
                       K_WORLD_47, K_WORLD_46, K_WORLD_45, K_WORLD_44,
                       K_WORLD_43, K_WORLD_42, K_WORLD_41, K_WORLD_40,
                       K_WORLD_39, K_WORLD_38, K_WORLD_37, K_WORLD_36,
                       K_WORLD_35, K_WORLD_34, K_WORLD_33, K_WORLD_32,
                       K_WORLD_31, K_WORLD_30, K_WORLD_29, K_WORLD_28,
                       K_WORLD_27, K_WORLD_26, K_WORLD_25, K_WORLD_24,
                       K_WORLD_23, K_WORLD_22, K_WORLD_21, K_WORLD_20,
                       K_WORLD_19, K_WORLD_18, K_WORLD_17, K_WORLD_16,
                       K_WORLD_15, K_WORLD_14, K_WORLD_13, K_WORLD_12,
                       K_WORLD_11, K_WORLD_10, K_WORLD_9, K_WORLD_8,
                       K_WORLD_7, K_WORLD_6, K_WORLD_5, K_WORLD_4, K_WORLD_3,
                       K_WORLD_2, K_WORLD_1, K_WORLD_0, K_AUX16, K_AUX15,
                       K_AUX14, K_AUX13, K_AUX12, K_AUX11, K_AUX10, K_AUX9,
                       K_AUX8, K_AUX7, K_AUX6, K_AUX5, K_AUX4, K_AUX3, K_AUX2,
                       K_JOY32, K_JOY31, K_JOY30, K_JOY29, K_JOY28, K_JOY27,
                       K_JOY26, K_JOY25, K_JOY24, K_JOY23, K_JOY22, K_JOY21,
                       K_JOY20, K_JOY19, K_JOY18, K_JOY17, K_JOY16, K_JOY15,
                       K_JOY14, K_JOY13, K_JOY12, K_JOY11, K_JOY10, K_JOY9,
                       K_JOY8, K_JOY7, K_JOY6, K_JOY5, K_JOY4, K_JOY3, K_JOY2,
                       K_JOY1, K_KP_EQUALS, K_KP_STAR, K_KP_NUMLOCK,
                       K_KP_PLUS, K_KP_MINUS, K_KP_SLASH, K_KP_DEL, K_KP_INS,
                       K_KP_ENTER, K_KP_PGDN, K_KP_DOWNARROW, K_KP_END,
                       K_KP_RIGHTARROW, K_KP_5, K_KP_LEFTARROW, K_KP_PGUP,
                       K_KP_UPARROW, K_KP_HOME, K_F15, K_F14, K_F13, K_F12,
                       K_F11, K_F10, K_F9, K_F8, K_F7, K_F6, K_F5, K_F4, K_F3,
                       K_F2, K_F1, K_END, K_HOME, K_PGUP, K_PGDN, K_DEL,
                       K_INS, K_SHIFT, K_CTRL, K_ALT, K_RIGHTARROW,
                       K_LEFTARROW, K_DOWNARROW, K_UPARROW, K_PAUSE, K_POWER,
                       K_CAPSLOCK, K_COMMAND, K_BACKSPACE, K_SPACE, K_ESCAPE,
                       K_ENTER, K_TAB};
use self::keys_h::{qkey_t, keys, Key_GetBinding};
use self::opus_h::{OpusEncoder, OpusDecoder};
use self::multi_h::{CURLM};
use self::curl_h::{CURL};
use self::stdlib_h::{abs};
use self::string_h::{memset};
use self::SDL_error_h::{SDL_GetError};
use self::SDL_mouse_h::{SDL_WarpMouseInWindow, SDL_SetRelativeMouseMode,
                        SDL_ShowCursor};
use self::SDL_h::{SDL_Init, SDL_QuitSubSystem, SDL_WasInit};
//
// input interface
//
#[no_mangle]
pub unsafe extern "C" fn IN_Init(mut windowData: *mut libc::c_void) {
    let mut appState: libc::c_int = 0;
    if 0 == SDL_WasInit(0x20u32) {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"IN_Init called before SDL_Init( SDL_INIT_VIDEO )\x00" as
                      *const u8 as *const libc::c_char);
    }
    SDL_window = windowData as *mut SDL_Window;
    Com_DPrintf(b"\n------- Input Initialization -------\n\x00" as *const u8
                    as *const libc::c_char);
    in_keyboardDebug =
        Cvar_Get(b"in_keyboardDebug\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x1i32);
    in_mouse =
        Cvar_Get(b"in_mouse\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char, 0x1i32);
    in_nograb =
        Cvar_Get(b"in_nograb\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x1i32);
    in_joystick =
        Cvar_Get(b"in_joystick\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 0x1i32 | 0x20i32);
    in_joystickThreshold =
        Cvar_Get(b"joy_threshold\x00" as *const u8 as *const libc::c_char,
                 b"0.15\x00" as *const u8 as *const libc::c_char, 0x1i32);
    SDL_StartTextInput();
    mouseAvailable =
        ((*in_mouse).value != 0i32 as libc::c_float) as libc::c_int as
            qboolean;
    IN_DeactivateMouse((Cvar_VariableIntegerValue(b"r_fullscreen\x00" as
                                                      *const u8 as
                                                      *const libc::c_char) !=
                            0i32) as libc::c_int as qboolean);
    appState = SDL_GetWindowFlags(SDL_window) as libc::c_int;
    Cvar_SetValue(b"com_unfocused\x00" as *const u8 as *const libc::c_char,
                  (0 == appState & SDL_WINDOW_INPUT_FOCUS as libc::c_int) as
                      libc::c_int as libc::c_float);
    Cvar_SetValue(b"com_minimized\x00" as *const u8 as *const libc::c_char,
                  (appState & SDL_WINDOW_MINIMIZED as libc::c_int) as
                      libc::c_float);
    IN_InitJoystick();
    Com_DPrintf(b"------------------------------------\n\x00" as *const u8 as
                    *const libc::c_char);
}
/*
===============
IN_InitJoystick
===============
*/
unsafe extern "C" fn IN_InitJoystick() {
    let mut i: libc::c_int = 0i32;
    let mut total: libc::c_int = 0i32;
    let mut buf: [libc::c_char; 16384] =
        *::std::mem::transmute::<&[u8; 16384],
                                 &mut [libc::c_char; 16384]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    if !gamepad.is_null() { SDL_GameControllerClose(gamepad); }
    if !stick.is_null() { SDL_JoystickClose(stick); }
    stick = 0 as *mut SDL_Joystick;
    gamepad = 0 as *mut SDL_GameController;
    memset(&mut stick_state as *mut unnamed_6 as *mut libc::c_void,
           '\u{0}' as i32,
           ::std::mem::size_of::<unnamed_6>() as libc::c_ulong);
    if 0 == SDL_WasInit(0x200u32) {
        Com_DPrintf(b"Calling SDL_Init(SDL_INIT_JOYSTICK)...\n\x00" as
                        *const u8 as *const libc::c_char);
        if SDL_Init(0x200u32) != 0i32 {
            Com_DPrintf(b"SDL_Init(SDL_INIT_JOYSTICK) failed: %s\n\x00" as
                            *const u8 as *const libc::c_char, SDL_GetError());
            return
        }
        Com_DPrintf(b"SDL_Init(SDL_INIT_JOYSTICK) passed.\n\x00" as *const u8
                        as *const libc::c_char);
    }
    if 0 == SDL_WasInit(0x2000u32) {
        Com_DPrintf(b"Calling SDL_Init(SDL_INIT_GAMECONTROLLER)...\n\x00" as
                        *const u8 as *const libc::c_char);
        if SDL_Init(0x2000u32) != 0i32 {
            Com_DPrintf(b"SDL_Init(SDL_INIT_GAMECONTROLLER) failed: %s\n\x00"
                            as *const u8 as *const libc::c_char,
                        SDL_GetError());
            return
        }
        Com_DPrintf(b"SDL_Init(SDL_INIT_GAMECONTROLLER) passed.\n\x00" as
                        *const u8 as *const libc::c_char);
    }
    total = SDL_NumJoysticks();
    Com_DPrintf(b"%d possible joysticks\n\x00" as *const u8 as
                    *const libc::c_char, total);
    i = 0i32;
    while i < total {
        Q_strcat(buf.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 16384]>() as
                     libc::c_ulong as libc::c_int,
                 SDL_JoystickNameForIndex(i));
        Q_strcat(buf.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 16384]>() as
                     libc::c_ulong as libc::c_int,
                 b"\n\x00" as *const u8 as *const libc::c_char);
        i += 1
    }
    Cvar_Get(b"in_availableJoysticks\x00" as *const u8 as *const libc::c_char,
             buf.as_mut_ptr(), 0x40i32);
    if 0 == (*in_joystick).integer {
        Com_DPrintf(b"Joystick is not active.\n\x00" as *const u8 as
                        *const libc::c_char);
        SDL_QuitSubSystem(0x2000u32);
        return
    }
    in_joystickNo =
        Cvar_Get(b"in_joystickNo\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x1i32);
    if (*in_joystickNo).integer < 0i32 || (*in_joystickNo).integer >= total {
        Cvar_Set(b"in_joystickNo\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char);
    }
    in_joystickUseAnalog =
        Cvar_Get(b"in_joystickUseAnalog\x00" as *const u8 as
                     *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x1i32);
    stick = SDL_JoystickOpen((*in_joystickNo).integer);
    if stick.is_null() {
        Com_DPrintf(b"No joystick opened: %s\n\x00" as *const u8 as
                        *const libc::c_char, SDL_GetError());
        return
    }
    if 0 != SDL_IsGameController((*in_joystickNo).integer) as u64 {
        gamepad = SDL_GameControllerOpen((*in_joystickNo).integer)
    }
    Com_DPrintf(b"Joystick %d opened\n\x00" as *const u8 as
                    *const libc::c_char, (*in_joystickNo).integer);
    Com_DPrintf(b"Name:       %s\n\x00" as *const u8 as *const libc::c_char,
                SDL_JoystickNameForIndex((*in_joystickNo).integer));
    Com_DPrintf(b"Axes:       %d\n\x00" as *const u8 as *const libc::c_char,
                SDL_JoystickNumAxes(stick));
    Com_DPrintf(b"Hats:       %d\n\x00" as *const u8 as *const libc::c_char,
                SDL_JoystickNumHats(stick));
    Com_DPrintf(b"Buttons:    %d\n\x00" as *const u8 as *const libc::c_char,
                SDL_JoystickNumButtons(stick));
    Com_DPrintf(b"Balls:      %d\n\x00" as *const u8 as *const libc::c_char,
                SDL_JoystickNumBalls(stick));
    Com_DPrintf(b"Use Analog: %s\n\x00" as *const u8 as *const libc::c_char,
                if 0 != (*in_joystickUseAnalog).integer {
                    b"Yes\x00" as *const u8 as *const libc::c_char
                } else { b"No\x00" as *const u8 as *const libc::c_char });
    Com_DPrintf(b"Is gamepad: %s\n\x00" as *const u8 as *const libc::c_char,
                if !gamepad.is_null() {
                    b"Yes\x00" as *const u8 as *const libc::c_char
                } else { b"No\x00" as *const u8 as *const libc::c_char });
    SDL_JoystickEventState(-1i32);
    SDL_GameControllerEventState(-1i32);
}
static mut gamepad: *mut SDL_GameController =
    0 as *const SDL_GameController as *mut SDL_GameController;
static mut in_joystickUseAnalog: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
static mut stick: *mut SDL_Joystick =
    0 as *const SDL_Joystick as *mut SDL_Joystick;
static mut in_joystickNo: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut in_joystick: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut stick_state: unnamed_6 =
    unnamed_6{buttons: [qfalse; 16],
              oldaxes: 0,
              oldaaxes: [0; 16],
              oldhats: 0,};
static mut SDL_window: *mut SDL_Window =
    0 as *const SDL_Window as *mut SDL_Window;
/*
===============
IN_DeactivateMouse
===============
*/
unsafe extern "C" fn IN_DeactivateMouse(mut isFullscreen: qboolean) {
    if 0 == SDL_WasInit(0x20u32) { return }
    if 0 == isFullscreen as u64 { SDL_ShowCursor(SDL_TRUE as libc::c_int); }
    if 0 == mouseAvailable as u64 { return }
    if 0 != mouseActive as u64 {
        IN_GobbleMotionEvents();
        SDL_SetWindowGrab(SDL_window, SDL_FALSE);
        SDL_SetRelativeMouseMode(SDL_FALSE);
        if 0 !=
               SDL_GetWindowFlags(SDL_window) &
                   SDL_WINDOW_MOUSE_FOCUS as libc::c_int as libc::c_uint {
            SDL_WarpMouseInWindow(SDL_window, cls.glconfig.vidWidth / 2i32,
                                  cls.glconfig.vidHeight / 2i32);
        }
        mouseActive = qfalse
    };
}
static mut mouseActive: qboolean = qfalse;
/*
===============
IN_GobbleMotionEvents
===============
*/
unsafe extern "C" fn IN_GobbleMotionEvents() {
    let mut dummy: [SDL_Event; 1] = [SDL_Event{type_0: 0,}; 1];
    let mut val: libc::c_int = 0i32;
    SDL_PumpEvents();
    loop  {
        val =
            SDL_PeepEvents(dummy.as_mut_ptr(), 1i32, SDL_GETEVENT,
                           SDL_MOUSEMOTION as libc::c_int as Uint32,
                           SDL_MOUSEMOTION as libc::c_int as Uint32);
        if !(val > 0i32) { break ; }
    }
    if val < 0i32 {
        Com_Printf(b"IN_GobbleMotionEvents failed: %s\n\x00" as *const u8 as
                       *const libc::c_char, SDL_GetError());
    };
}
static mut mouseAvailable: qboolean = qfalse;
static mut in_mouse: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut in_joystickThreshold: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
static mut in_nograb: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
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
static mut in_keyboardDebug: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn IN_Frame() {
    let mut loading: qboolean = qfalse;
    IN_JoyMove();
    loading =
        (clc.state as libc::c_uint !=
             CA_DISCONNECTED as libc::c_int as libc::c_uint &&
             clc.state as libc::c_uint !=
                 CA_ACTIVE as libc::c_int as libc::c_uint) as libc::c_int as
            qboolean;
    cls.glconfig.isFullscreen =
        (Cvar_VariableIntegerValue(b"r_fullscreen\x00" as *const u8 as
                                       *const libc::c_char) != 0i32) as
            libc::c_int as qboolean;
    if 0 == cls.glconfig.isFullscreen as u64 && 0 != Key_GetCatcher() & 0x1i32
       {
        IN_DeactivateMouse(cls.glconfig.isFullscreen);
    } else if 0 == cls.glconfig.isFullscreen as u64 &&
                  0 != loading as libc::c_uint {
        IN_DeactivateMouse(cls.glconfig.isFullscreen);
    } else if 0 ==
                  SDL_GetWindowFlags(SDL_window) &
                      SDL_WINDOW_INPUT_FOCUS as libc::c_int as libc::c_uint {
        IN_DeactivateMouse(cls.glconfig.isFullscreen);
    } else { IN_ActivateMouse(cls.glconfig.isFullscreen); }
    IN_ProcessEvents();
    in_eventTime = Sys_Milliseconds();
    if vidRestartTime != 0i32 && vidRestartTime < Sys_Milliseconds() {
        vidRestartTime = 0i32;
        Cbuf_AddText(b"vid_restart\n\x00" as *const u8 as
                         *const libc::c_char);
    };
}
static mut vidRestartTime: libc::c_int = 0i32;
static mut in_eventTime: libc::c_int = 0i32;
/*
===============
IN_ProcessEvents
===============
*/
unsafe extern "C" fn IN_ProcessEvents() {
    let mut e: SDL_Event = SDL_Event{type_0: 0,};
    let mut key: keyNum_t = 0 as keyNum_t;
    static mut lastKeyDown: keyNum_t = 0 as keyNum_t;
    if 0 == SDL_WasInit(0x20u32) { return }
    while 0 != SDL_PollEvent(&mut e) {
        match e.type_0 {
            768 => {
                if !(0 != e.key.repeat as libc::c_int &&
                         Key_GetCatcher() == 0i32) {
                    key = IN_TranslateSDLToQ3Key(&mut e.key.keysym, qtrue);
                    if 0 != key as u64 {
                        Com_QueueEvent(in_eventTime, SE_KEY,
                                       key as libc::c_int,
                                       qtrue as libc::c_int, 0i32,
                                       0 as *mut libc::c_void);
                    }
                    if key as libc::c_uint ==
                           K_BACKSPACE as libc::c_int as libc::c_uint {
                        Com_QueueEvent(in_eventTime, SE_CHAR,
                                       'h' as i32 - 'a' as i32 + 1i32, 0i32,
                                       0i32, 0 as *mut libc::c_void);
                    } else if 0 !=
                                  keys[K_CTRL as libc::c_int as usize].down as
                                      libc::c_uint &&
                                  key as libc::c_uint >=
                                      'a' as i32 as libc::c_uint &&
                                  key as libc::c_uint <=
                                      'z' as i32 as libc::c_uint {
                        Com_QueueEvent(in_eventTime, SE_CHAR,
                                       (key as
                                            libc::c_uint).wrapping_sub('a' as
                                                                           i32
                                                                           as
                                                                           libc::c_uint).wrapping_add(1i32
                                                                                                          as
                                                                                                          libc::c_uint)
                                           as libc::c_int, 0i32, 0i32,
                                       0 as *mut libc::c_void);
                    }
                    lastKeyDown = key
                }
            }
            769 => {
                key = IN_TranslateSDLToQ3Key(&mut e.key.keysym, qfalse);
                if 0 != key as u64 {
                    Com_QueueEvent(in_eventTime, SE_KEY, key as libc::c_int,
                                   qfalse as libc::c_int, 0i32,
                                   0 as *mut libc::c_void);
                }
                lastKeyDown = 0 as keyNum_t
            }
            771 => {
                if lastKeyDown as libc::c_uint !=
                       K_CONSOLE as libc::c_int as libc::c_uint {
                    let mut c: *mut libc::c_char = e.text.text.as_mut_ptr();
                    while 0 != *c {
                        let mut utf32: libc::c_int = 0i32;
                        if *c as libc::c_int & 0x80i32 == 0i32 {
                            let fresh0 = c;
                            c = c.offset(1);
                            utf32 = *fresh0 as libc::c_int
                        } else if *c as libc::c_int & 0xe0i32 == 0xc0i32 {
                            let fresh1 = c;
                            c = c.offset(1);
                            utf32 |=
                                (*fresh1 as libc::c_int & 0x1fi32) << 6i32;
                            let fresh2 = c;
                            c = c.offset(1);
                            utf32 |= *fresh2 as libc::c_int & 0x3fi32
                        } else if *c as libc::c_int & 0xf0i32 == 0xe0i32 {
                            let fresh3 = c;
                            c = c.offset(1);
                            utf32 |=
                                (*fresh3 as libc::c_int & 0xfi32) << 12i32;
                            let fresh4 = c;
                            c = c.offset(1);
                            utf32 |=
                                (*fresh4 as libc::c_int & 0x3fi32) << 6i32;
                            let fresh5 = c;
                            c = c.offset(1);
                            utf32 |= *fresh5 as libc::c_int & 0x3fi32
                        } else if *c as libc::c_int & 0xf8i32 == 0xf0i32 {
                            let fresh6 = c;
                            c = c.offset(1);
                            utf32 |=
                                (*fresh6 as libc::c_int & 0x7i32) << 18i32;
                            let fresh7 = c;
                            c = c.offset(1);
                            utf32 |=
                                (*fresh7 as libc::c_int & 0x3fi32) << 12i32;
                            let fresh8 = c;
                            c = c.offset(1);
                            utf32 |=
                                (*fresh8 as libc::c_int & 0x3fi32) << 6i32;
                            let fresh9 = c;
                            c = c.offset(1);
                            utf32 |= *fresh9 as libc::c_int & 0x3fi32
                        } else {
                            Com_DPrintf(b"Unrecognised UTF-8 lead byte: 0x%x\n\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        *c as libc::c_uint);
                            c = c.offset(1isize)
                        }
                        if utf32 != 0i32 {
                            if 0 !=
                                   IN_IsConsoleKey(0 as keyNum_t, utf32) as
                                       u64 {
                                Com_QueueEvent(in_eventTime, SE_KEY,
                                               K_CONSOLE as libc::c_int,
                                               qtrue as libc::c_int, 0i32,
                                               0 as *mut libc::c_void);
                                Com_QueueEvent(in_eventTime, SE_KEY,
                                               K_CONSOLE as libc::c_int,
                                               qfalse as libc::c_int, 0i32,
                                               0 as *mut libc::c_void);
                            } else {
                                Com_QueueEvent(in_eventTime, SE_CHAR, utf32,
                                               0i32, 0i32,
                                               0 as *mut libc::c_void);
                            }
                        }
                    }
                }
            }
            1024 => {
                if 0 != mouseActive as u64 {
                    if !(0 == e.motion.xrel && 0 == e.motion.yrel) {
                        Com_QueueEvent(in_eventTime, SE_MOUSE, e.motion.xrel,
                                       e.motion.yrel, 0i32,
                                       0 as *mut libc::c_void);
                    }
                }
            }
            1025 | 1026 => {
                let mut b: libc::c_int = 0;
                match e.button.button as libc::c_int {
                    1 => { b = K_MOUSE1 as libc::c_int }
                    2 => { b = K_MOUSE3 as libc::c_int }
                    3 => { b = K_MOUSE2 as libc::c_int }
                    4 => { b = K_MOUSE4 as libc::c_int }
                    5 => { b = K_MOUSE5 as libc::c_int }
                    _ => {
                        b =
                            K_AUX1 as libc::c_int +
                                (e.button.button as libc::c_int - 5i32 + 1i32)
                                    % 16i32
                    }
                }
                Com_QueueEvent(in_eventTime, SE_KEY, b,
                               if e.type_0 ==
                                      SDL_MOUSEBUTTONDOWN as libc::c_int as
                                          libc::c_uint {
                                   qtrue as libc::c_int
                               } else { qfalse as libc::c_int }, 0i32,
                               0 as *mut libc::c_void);
            }
            1027 => {
                if e.wheel.y > 0i32 {
                    Com_QueueEvent(in_eventTime, SE_KEY,
                                   K_MWHEELUP as libc::c_int,
                                   qtrue as libc::c_int, 0i32,
                                   0 as *mut libc::c_void);
                    Com_QueueEvent(in_eventTime, SE_KEY,
                                   K_MWHEELUP as libc::c_int,
                                   qfalse as libc::c_int, 0i32,
                                   0 as *mut libc::c_void);
                } else if e.wheel.y < 0i32 {
                    Com_QueueEvent(in_eventTime, SE_KEY,
                                   K_MWHEELDOWN as libc::c_int,
                                   qtrue as libc::c_int, 0i32,
                                   0 as *mut libc::c_void);
                    Com_QueueEvent(in_eventTime, SE_KEY,
                                   K_MWHEELDOWN as libc::c_int,
                                   qfalse as libc::c_int, 0i32,
                                   0 as *mut libc::c_void);
                }
            }
            1619 | 1620 => {
                if 0 != (*in_joystick).integer { IN_InitJoystick(); }
            }
            256 => {
                Cbuf_ExecuteText(EXEC_NOW as libc::c_int,
                                 b"quit Closed window\n\x00" as *const u8 as
                                     *const libc::c_char);
            }
            512 => {
                match e.window.event as libc::c_int {
                    5 => {
                        let mut width: libc::c_int = 0;
                        let mut height: libc::c_int = 0;
                        width = e.window.data1;
                        height = e.window.data2;
                        // ignore this event on fullscreen
                        if !(0 != cls.glconfig.isFullscreen as u64) {
                            // check if size actually changed
                            if !(cls.glconfig.vidWidth == width &&
                                     cls.glconfig.vidHeight == height) {
                                Cvar_SetValue(b"r_customwidth\x00" as
                                                  *const u8 as
                                                  *const libc::c_char,
                                              width as libc::c_float);
                                Cvar_SetValue(b"r_customheight\x00" as
                                                  *const u8 as
                                                  *const libc::c_char,
                                              height as libc::c_float);
                                Cvar_Set(b"r_mode\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"-1\x00" as *const u8 as
                                             *const libc::c_char);
                                vidRestartTime = Sys_Milliseconds() + 1000i32
                            }
                        }
                    }
                    7 => {
                        Cvar_SetValue(b"com_minimized\x00" as *const u8 as
                                          *const libc::c_char,
                                      1i32 as libc::c_float);
                    }
                    9 | 8 => {
                        Cvar_SetValue(b"com_minimized\x00" as *const u8 as
                                          *const libc::c_char,
                                      0i32 as libc::c_float);
                    }
                    13 => {
                        Cvar_SetValue(b"com_unfocused\x00" as *const u8 as
                                          *const libc::c_char,
                                      1i32 as libc::c_float);
                    }
                    12 => {
                        Cvar_SetValue(b"com_unfocused\x00" as *const u8 as
                                          *const libc::c_char,
                                      0i32 as libc::c_float);
                    }
                    _ => { }
                }
            }
            _ => { }
        }
    };
}
/*
===============
IN_IsConsoleKey

TODO: If the SDL_Scancode situation improves, use it instead of
      both of these methods
===============
*/
unsafe extern "C" fn IN_IsConsoleKey(mut key: keyNum_t,
                                     mut character: libc::c_int) -> qboolean {
    static mut consoleKeys: [consoleKey_t; 16] =
        [consoleKey_s{type_0: QUAKE_KEY, u: unnamed_7{key: 0 as keyNum_t,},};
            16];
    static mut numConsoleKeys: libc::c_int = 0i32;
    let mut i: libc::c_int = 0;
    if 0 != (*cl_consoleKeys).modified as u64 {
        let mut text_p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
        (*cl_consoleKeys).modified = qfalse;
        text_p = (*cl_consoleKeys).string;
        numConsoleKeys = 0i32;
        while numConsoleKeys < 16i32 {
            let mut c: *mut consoleKey_t =
                &mut *consoleKeys.as_mut_ptr().offset(numConsoleKeys as isize)
                    as *mut consoleKey_t;
            let mut charCode: libc::c_int = 0i32;
            token = COM_Parse(&mut text_p);
            if 0 == *token.offset(0isize) { break ; }
            charCode = Com_HexStrToInt(token);
            if charCode > 0i32 {
                (*c).type_0 = CHARACTER;
                (*c).u.character = charCode
            } else {
                (*c).type_0 = QUAKE_KEY;
                (*c).u.key = Key_StringToKeynum(token) as keyNum_t;
                // 0 isn't a key
                if (*c).u.key as libc::c_uint <= 0i32 as libc::c_uint {
                    continue ;
                }
            }
            numConsoleKeys += 1
        }
    }
    if key as libc::c_uint == character as libc::c_uint {
        key = 0 as keyNum_t
    }
    i = 0i32;
    while i < numConsoleKeys {
        let mut c_0: *mut consoleKey_t =
            &mut *consoleKeys.as_mut_ptr().offset(i as isize) as
                *mut consoleKey_t;
        match (*c_0).type_0 as libc::c_uint {
            0 => {
                if 0 != key as libc::c_uint &&
                       (*c_0).u.key as libc::c_uint == key as libc::c_uint {
                    return qtrue
                }
            }
            1 => { if (*c_0).u.character == character { return qtrue } }
            _ => { }
        }
        i += 1
    }
    return qfalse;
}
/*
===============
IN_TranslateSDLToQ3Key
===============
*/
unsafe extern "C" fn IN_TranslateSDLToQ3Key(mut keysym: *mut SDL_Keysym,
                                            mut down: qboolean) -> keyNum_t {
    let mut key: keyNum_t = 0 as keyNum_t;
    if (*keysym).scancode as libc::c_uint >=
           SDL_SCANCODE_1 as libc::c_int as libc::c_uint &&
           (*keysym).scancode as libc::c_uint <=
               SDL_SCANCODE_0 as libc::c_int as libc::c_uint {
        if (*keysym).scancode as libc::c_uint ==
               SDL_SCANCODE_0 as libc::c_int as libc::c_uint {
            key = '0' as i32 as keyNum_t
        } else {
            key =
                ('1' as i32 as
                     libc::c_uint).wrapping_add((*keysym).scancode as
                                                    libc::c_uint).wrapping_sub(SDL_SCANCODE_1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint)
                    as keyNum_t
        }
    } else if (*keysym).sym >= SDLK_SPACE as libc::c_int &&
                  (*keysym).sym < SDLK_DELETE as libc::c_int {
        key = (*keysym).sym as keyNum_t
    } else {
        match (*keysym).sym {
            1073741899 => { key = K_PGUP }
            1073741921 => { key = K_KP_PGUP }
            1073741902 => { key = K_PGDN }
            1073741915 => { key = K_KP_PGDN }
            1073741919 => { key = K_KP_HOME }
            1073741898 => { key = K_HOME }
            1073741913 => { key = K_KP_END }
            1073741901 => { key = K_END }
            1073741916 => { key = K_KP_LEFTARROW }
            1073741904 => { key = K_LEFTARROW }
            1073741918 => { key = K_KP_RIGHTARROW }
            1073741903 => { key = K_RIGHTARROW }
            1073741914 => { key = K_KP_DOWNARROW }
            1073741905 => { key = K_DOWNARROW }
            1073741920 => { key = K_KP_UPARROW }
            1073741906 => { key = K_UPARROW }
            27 => { key = K_ESCAPE }
            1073741912 => { key = K_KP_ENTER }
            13 => { key = K_ENTER }
            9 => { key = K_TAB }
            1073741882 => { key = K_F1 }
            1073741883 => { key = K_F2 }
            1073741884 => { key = K_F3 }
            1073741885 => { key = K_F4 }
            1073741886 => { key = K_F5 }
            1073741887 => { key = K_F6 }
            1073741888 => { key = K_F7 }
            1073741889 => { key = K_F8 }
            1073741890 => { key = K_F9 }
            1073741891 => { key = K_F10 }
            1073741892 => { key = K_F11 }
            1073741893 => { key = K_F12 }
            1073741928 => { key = K_F13 }
            1073741929 => { key = K_F14 }
            1073741930 => { key = K_F15 }
            8 => { key = K_BACKSPACE }
            1073741923 => { key = K_KP_DEL }
            127 => { key = K_DEL }
            1073741896 => { key = K_PAUSE }
            1073742049 | 1073742053 => { key = K_SHIFT }
            1073742048 | 1073742052 => { key = K_CTRL }
            1073742055 | 1073742051 => { key = K_SUPER }
            1073742054 | 1073742050 => { key = K_ALT }
            1073741917 => { key = K_KP_5 }
            1073741897 => { key = K_INS }
            1073741922 => { key = K_KP_INS }
            1073741909 => { key = K_KP_STAR }
            1073741911 => { key = K_KP_PLUS }
            1073741910 => { key = K_KP_MINUS }
            1073741908 => { key = K_KP_SLASH }
            1073742081 => { key = K_MODE }
            1073741941 => { key = K_HELP }
            1073741894 => { key = K_PRINT }
            1073741978 => { key = K_SYSREQ }
            1073741942 => { key = K_MENU }
            1073741925 => { key = K_MENU }
            1073741926 => { key = K_POWER }
            1073741946 => { key = K_UNDO }
            1073741895 => { key = K_SCROLLOCK }
            1073741907 => { key = K_KP_NUMLOCK }
            1073741881 => { key = K_CAPSLOCK }
            _ => {
                if 0 == (*keysym).sym & 1i32 << 30i32 &&
                       (*keysym).scancode as libc::c_uint <=
                           95i32 as libc::c_uint {
                    key =
                        (K_WORLD_0 as libc::c_int +
                             (*keysym).scancode as libc::c_int) as keyNum_t
                }
            }
        }
    }
    if 0 != (*in_keyboardDebug).integer { IN_PrintKey(keysym, key, down); }
    if 0 != IN_IsConsoleKey(key, 0i32) as u64 { key = K_CONSOLE }
    return key;
}
/*
===============
IN_PrintKey
===============
*/
unsafe extern "C" fn IN_PrintKey(mut keysym: *const SDL_Keysym,
                                 mut key: keyNum_t, mut down: qboolean) {
    if 0 != down as u64 {
        Com_Printf(b"+ \x00" as *const u8 as *const libc::c_char);
    } else { Com_Printf(b"  \x00" as *const u8 as *const libc::c_char); }
    Com_Printf(b"Scancode: 0x%02x(%s) Sym: 0x%02x(%s)\x00" as *const u8 as
                   *const libc::c_char, (*keysym).scancode as libc::c_uint,
               SDL_GetScancodeName((*keysym).scancode), (*keysym).sym,
               SDL_GetKeyName((*keysym).sym));
    if 0 != (*keysym).mod_0 as libc::c_int & KMOD_LSHIFT as libc::c_int {
        Com_Printf(b" KMOD_LSHIFT\x00" as *const u8 as *const libc::c_char);
    }
    if 0 != (*keysym).mod_0 as libc::c_int & KMOD_RSHIFT as libc::c_int {
        Com_Printf(b" KMOD_RSHIFT\x00" as *const u8 as *const libc::c_char);
    }
    if 0 != (*keysym).mod_0 as libc::c_int & KMOD_LCTRL as libc::c_int {
        Com_Printf(b" KMOD_LCTRL\x00" as *const u8 as *const libc::c_char);
    }
    if 0 != (*keysym).mod_0 as libc::c_int & KMOD_RCTRL as libc::c_int {
        Com_Printf(b" KMOD_RCTRL\x00" as *const u8 as *const libc::c_char);
    }
    if 0 != (*keysym).mod_0 as libc::c_int & KMOD_LALT as libc::c_int {
        Com_Printf(b" KMOD_LALT\x00" as *const u8 as *const libc::c_char);
    }
    if 0 != (*keysym).mod_0 as libc::c_int & KMOD_RALT as libc::c_int {
        Com_Printf(b" KMOD_RALT\x00" as *const u8 as *const libc::c_char);
    }
    if 0 != (*keysym).mod_0 as libc::c_int & KMOD_LGUI as libc::c_int {
        Com_Printf(b" KMOD_LGUI\x00" as *const u8 as *const libc::c_char);
    }
    if 0 != (*keysym).mod_0 as libc::c_int & KMOD_RGUI as libc::c_int {
        Com_Printf(b" KMOD_RGUI\x00" as *const u8 as *const libc::c_char);
    }
    if 0 != (*keysym).mod_0 as libc::c_int & KMOD_NUM as libc::c_int {
        Com_Printf(b" KMOD_NUM\x00" as *const u8 as *const libc::c_char);
    }
    if 0 != (*keysym).mod_0 as libc::c_int & KMOD_CAPS as libc::c_int {
        Com_Printf(b" KMOD_CAPS\x00" as *const u8 as *const libc::c_char);
    }
    if 0 != (*keysym).mod_0 as libc::c_int & KMOD_MODE as libc::c_int {
        Com_Printf(b" KMOD_MODE\x00" as *const u8 as *const libc::c_char);
    }
    if 0 != (*keysym).mod_0 as libc::c_int & KMOD_RESERVED as libc::c_int {
        Com_Printf(b" KMOD_RESERVED\x00" as *const u8 as *const libc::c_char);
    }
    Com_Printf(b" Q:0x%02x(%s)\n\x00" as *const u8 as *const libc::c_char,
               key as libc::c_uint, Key_KeynumToString(key as libc::c_int));
}
/*
===============
IN_ActivateMouse
===============
*/
unsafe extern "C" fn IN_ActivateMouse(mut isFullscreen: qboolean) {
    if 0 == mouseAvailable as u64 || 0 == SDL_WasInit(0x20u32) { return }
    if 0 == mouseActive as u64 {
        SDL_SetRelativeMouseMode(SDL_TRUE);
        SDL_SetWindowGrab(SDL_window, SDL_TRUE);
        IN_GobbleMotionEvents();
    }
    if 0 == isFullscreen as u64 {
        if 0 != (*in_nograb).modified as libc::c_uint ||
               0 == mouseActive as u64 {
            if 0 != (*in_nograb).integer {
                SDL_SetRelativeMouseMode(SDL_FALSE);
                SDL_SetWindowGrab(SDL_window, SDL_FALSE);
            } else {
                SDL_SetRelativeMouseMode(SDL_TRUE);
                SDL_SetWindowGrab(SDL_window, SDL_TRUE);
            }
            (*in_nograb).modified = qfalse
        }
    }
    mouseActive = qtrue;
}
/*
===============
IN_JoyMove
===============
*/
unsafe extern "C" fn IN_JoyMove() {
    let mut axes: libc::c_uint = 0i32 as libc::c_uint;
    let mut hats: libc::c_uint = 0i32 as libc::c_uint;
    let mut total: libc::c_int = 0i32;
    let mut i: libc::c_int = 0i32;
    if !gamepad.is_null() { IN_GamepadMove(); return }
    if stick.is_null() { return }
    SDL_JoystickUpdate();
    total = SDL_JoystickNumBalls(stick);
    if total > 0i32 {
        let mut balldx: libc::c_int = 0i32;
        let mut balldy: libc::c_int = 0i32;
        i = 0i32;
        while i < total {
            let mut dx: libc::c_int = 0i32;
            let mut dy: libc::c_int = 0i32;
            SDL_JoystickGetBall(stick, i, &mut dx, &mut dy);
            balldx += dx;
            balldy += dy;
            i += 1
        }
        if 0 != balldx || 0 != balldy {
            if abs(balldx) > 1i32 { balldx *= 2i32 }
            if abs(balldy) > 1i32 { balldy *= 2i32 }
            Com_QueueEvent(in_eventTime, SE_MOUSE, balldx, balldy, 0i32,
                           0 as *mut libc::c_void);
        }
    }
    total = SDL_JoystickNumButtons(stick);
    if total > 0i32 {
        if total as libc::c_ulong >
               (::std::mem::size_of::<[qboolean; 16]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<qboolean>()
                                                    as libc::c_ulong) {
            total =
                (::std::mem::size_of::<[qboolean; 16]>() as
                     libc::c_ulong).wrapping_div(::std::mem::size_of::<qboolean>()
                                                     as libc::c_ulong) as
                    libc::c_int
        }
        i = 0i32;
        while i < total {
            let mut pressed: qboolean =
                (SDL_JoystickGetButton(stick, i) as libc::c_int != 0i32) as
                    libc::c_int as qboolean;
            if pressed as libc::c_uint !=
                   stick_state.buttons[i as usize] as libc::c_uint {
                Com_QueueEvent(in_eventTime, SE_KEY,
                               K_JOY1 as libc::c_int + i,
                               pressed as libc::c_int, 0i32,
                               0 as *mut libc::c_void);
                stick_state.buttons[i as usize] = pressed
            }
            i += 1
        }
    }
    total = SDL_JoystickNumHats(stick);
    if total > 0i32 {
        if total > 4i32 { total = 4i32 }
        i = 0i32;
        while i < total {
            *(&mut hats as *mut libc::c_uint as *mut Uint8).offset(i as isize)
                = SDL_JoystickGetHat(stick, i);
            i += 1
        }
    }
    if hats != stick_state.oldhats {
        i = 0i32;
        while i < 4i32 {
            if *(&mut hats as *mut libc::c_uint as
                     *mut Uint8).offset(i as isize) as libc::c_int !=
                   *(&mut stick_state.oldhats as *mut libc::c_uint as
                         *mut Uint8).offset(i as isize) as libc::c_int {
                match *(&mut stick_state.oldhats as *mut libc::c_uint as
                            *mut Uint8).offset(i as isize) as libc::c_int {
                    1 => {
                        Com_QueueEvent(in_eventTime, SE_KEY,
                                       hat_keys[(4i32 * i + 0i32) as usize],
                                       qfalse as libc::c_int, 0i32,
                                       0 as *mut libc::c_void);
                    }
                    2 => {
                        Com_QueueEvent(in_eventTime, SE_KEY,
                                       hat_keys[(4i32 * i + 1i32) as usize],
                                       qfalse as libc::c_int, 0i32,
                                       0 as *mut libc::c_void);
                    }
                    4 => {
                        Com_QueueEvent(in_eventTime, SE_KEY,
                                       hat_keys[(4i32 * i + 2i32) as usize],
                                       qfalse as libc::c_int, 0i32,
                                       0 as *mut libc::c_void);
                    }
                    8 => {
                        Com_QueueEvent(in_eventTime, SE_KEY,
                                       hat_keys[(4i32 * i + 3i32) as usize],
                                       qfalse as libc::c_int, 0i32,
                                       0 as *mut libc::c_void);
                    }
                    3 => {
                        Com_QueueEvent(in_eventTime, SE_KEY,
                                       hat_keys[(4i32 * i + 0i32) as usize],
                                       qfalse as libc::c_int, 0i32,
                                       0 as *mut libc::c_void);
                        Com_QueueEvent(in_eventTime, SE_KEY,
                                       hat_keys[(4i32 * i + 1i32) as usize],
                                       qfalse as libc::c_int, 0i32,
                                       0 as *mut libc::c_void);
                    }
                    6 => {
                        Com_QueueEvent(in_eventTime, SE_KEY,
                                       hat_keys[(4i32 * i + 2i32) as usize],
                                       qfalse as libc::c_int, 0i32,
                                       0 as *mut libc::c_void);
                        Com_QueueEvent(in_eventTime, SE_KEY,
                                       hat_keys[(4i32 * i + 1i32) as usize],
                                       qfalse as libc::c_int, 0i32,
                                       0 as *mut libc::c_void);
                    }
                    9 => {
                        Com_QueueEvent(in_eventTime, SE_KEY,
                                       hat_keys[(4i32 * i + 0i32) as usize],
                                       qfalse as libc::c_int, 0i32,
                                       0 as *mut libc::c_void);
                        Com_QueueEvent(in_eventTime, SE_KEY,
                                       hat_keys[(4i32 * i + 3i32) as usize],
                                       qfalse as libc::c_int, 0i32,
                                       0 as *mut libc::c_void);
                    }
                    12 => {
                        Com_QueueEvent(in_eventTime, SE_KEY,
                                       hat_keys[(4i32 * i + 2i32) as usize],
                                       qfalse as libc::c_int, 0i32,
                                       0 as *mut libc::c_void);
                        Com_QueueEvent(in_eventTime, SE_KEY,
                                       hat_keys[(4i32 * i + 3i32) as usize],
                                       qfalse as libc::c_int, 0i32,
                                       0 as *mut libc::c_void);
                    }
                    _ => { }
                }
                match *(&mut hats as *mut libc::c_uint as
                            *mut Uint8).offset(i as isize) as libc::c_int {
                    1 => {
                        Com_QueueEvent(in_eventTime, SE_KEY,
                                       hat_keys[(4i32 * i + 0i32) as usize],
                                       qtrue as libc::c_int, 0i32,
                                       0 as *mut libc::c_void);
                    }
                    2 => {
                        Com_QueueEvent(in_eventTime, SE_KEY,
                                       hat_keys[(4i32 * i + 1i32) as usize],
                                       qtrue as libc::c_int, 0i32,
                                       0 as *mut libc::c_void);
                    }
                    4 => {
                        Com_QueueEvent(in_eventTime, SE_KEY,
                                       hat_keys[(4i32 * i + 2i32) as usize],
                                       qtrue as libc::c_int, 0i32,
                                       0 as *mut libc::c_void);
                    }
                    8 => {
                        Com_QueueEvent(in_eventTime, SE_KEY,
                                       hat_keys[(4i32 * i + 3i32) as usize],
                                       qtrue as libc::c_int, 0i32,
                                       0 as *mut libc::c_void);
                    }
                    3 => {
                        Com_QueueEvent(in_eventTime, SE_KEY,
                                       hat_keys[(4i32 * i + 0i32) as usize],
                                       qtrue as libc::c_int, 0i32,
                                       0 as *mut libc::c_void);
                        Com_QueueEvent(in_eventTime, SE_KEY,
                                       hat_keys[(4i32 * i + 1i32) as usize],
                                       qtrue as libc::c_int, 0i32,
                                       0 as *mut libc::c_void);
                    }
                    6 => {
                        Com_QueueEvent(in_eventTime, SE_KEY,
                                       hat_keys[(4i32 * i + 2i32) as usize],
                                       qtrue as libc::c_int, 0i32,
                                       0 as *mut libc::c_void);
                        Com_QueueEvent(in_eventTime, SE_KEY,
                                       hat_keys[(4i32 * i + 1i32) as usize],
                                       qtrue as libc::c_int, 0i32,
                                       0 as *mut libc::c_void);
                    }
                    9 => {
                        Com_QueueEvent(in_eventTime, SE_KEY,
                                       hat_keys[(4i32 * i + 0i32) as usize],
                                       qtrue as libc::c_int, 0i32,
                                       0 as *mut libc::c_void);
                        Com_QueueEvent(in_eventTime, SE_KEY,
                                       hat_keys[(4i32 * i + 3i32) as usize],
                                       qtrue as libc::c_int, 0i32,
                                       0 as *mut libc::c_void);
                    }
                    12 => {
                        Com_QueueEvent(in_eventTime, SE_KEY,
                                       hat_keys[(4i32 * i + 2i32) as usize],
                                       qtrue as libc::c_int, 0i32,
                                       0 as *mut libc::c_void);
                        Com_QueueEvent(in_eventTime, SE_KEY,
                                       hat_keys[(4i32 * i + 3i32) as usize],
                                       qtrue as libc::c_int, 0i32,
                                       0 as *mut libc::c_void);
                    }
                    _ => { }
                }
            }
            i += 1
        }
    }
    stick_state.oldhats = hats;
    total = SDL_JoystickNumAxes(stick);
    if total > 0i32 {
        if 0 != (*in_joystickUseAnalog).integer {
            if total > 16i32 { total = 16i32 }
            i = 0i32;
            while i < total {
                let mut axis: Sint16 = SDL_JoystickGetAxis(stick, i);
                let mut f: libc::c_float =
                    abs(axis as libc::c_int) as libc::c_float / 32767.0f32;
                if f < (*in_joystickThreshold).value { axis = 0i32 as Sint16 }
                if axis as libc::c_int != stick_state.oldaaxes[i as usize] {
                    Com_QueueEvent(in_eventTime, SE_JOYSTICK_AXIS, i,
                                   axis as libc::c_int, 0i32,
                                   0 as *mut libc::c_void);
                    stick_state.oldaaxes[i as usize] = axis as libc::c_int
                }
                i += 1
            }
        } else {
            if total > 16i32 { total = 16i32 }
            i = 0i32;
            while i < total {
                let mut axis_0: Sint16 = SDL_JoystickGetAxis(stick, i);
                let mut f_0: libc::c_float =
                    axis_0 as libc::c_float / 32767.0f32;
                if f_0 < -(*in_joystickThreshold).value {
                    axes |= (1i32 << i * 2i32) as libc::c_uint
                } else if f_0 > (*in_joystickThreshold).value {
                    axes |= (1i32 << i * 2i32 + 1i32) as libc::c_uint
                }
                i += 1
            }
        }
    }
    if axes != stick_state.oldaxes {
        i = 0i32;
        while i < 16i32 {
            if 0 != axes & (1i32 << i) as libc::c_uint &&
                   0 == stick_state.oldaxes & (1i32 << i) as libc::c_uint {
                Com_QueueEvent(in_eventTime, SE_KEY, joy_keys[i as usize],
                               qtrue as libc::c_int, 0i32,
                               0 as *mut libc::c_void);
            }
            if 0 == axes & (1i32 << i) as libc::c_uint &&
                   0 != stick_state.oldaxes & (1i32 << i) as libc::c_uint {
                Com_QueueEvent(in_eventTime, SE_KEY, joy_keys[i as usize],
                               qfalse as libc::c_int, 0i32,
                               0 as *mut libc::c_void);
            }
            i += 1
        }
    }
    stick_state.oldaxes = axes;
}
// We translate axes movement into keypresses
static mut joy_keys: [libc::c_int; 16] =
    [K_LEFTARROW as libc::c_int, K_RIGHTARROW as libc::c_int,
     K_UPARROW as libc::c_int, K_DOWNARROW as libc::c_int,
     K_JOY17 as libc::c_int, K_JOY18 as libc::c_int, K_JOY19 as libc::c_int,
     K_JOY20 as libc::c_int, K_JOY21 as libc::c_int, K_JOY22 as libc::c_int,
     K_JOY23 as libc::c_int, K_JOY24 as libc::c_int, K_JOY25 as libc::c_int,
     K_JOY26 as libc::c_int, K_JOY27 as libc::c_int, K_JOY28 as libc::c_int];
// translate hat events into keypresses
// the 4 highest buttons are used for the first hat ...
static mut hat_keys: [libc::c_int; 16] =
    [K_JOY29 as libc::c_int, K_JOY30 as libc::c_int, K_JOY31 as libc::c_int,
     K_JOY32 as libc::c_int, K_JOY25 as libc::c_int, K_JOY26 as libc::c_int,
     K_JOY27 as libc::c_int, K_JOY28 as libc::c_int, K_JOY21 as libc::c_int,
     K_JOY22 as libc::c_int, K_JOY23 as libc::c_int, K_JOY24 as libc::c_int,
     K_JOY17 as libc::c_int, K_JOY18 as libc::c_int, K_JOY19 as libc::c_int,
     K_JOY20 as libc::c_int];
/*
===============
IN_GamepadMove
===============
*/
unsafe extern "C" fn IN_GamepadMove() {
    let mut i: libc::c_int = 0;
    let mut translatedAxes: [libc::c_int; 16] = [0; 16];
    let mut translatedAxesSet: [qboolean; 16] = [qfalse; 16];
    SDL_GameControllerUpdate();
    i = 0i32;
    while i < SDL_CONTROLLER_BUTTON_MAX as libc::c_int {
        let mut pressed: qboolean =
            SDL_GameControllerGetButton(gamepad,
                                        (SDL_CONTROLLER_BUTTON_A as
                                             libc::c_int + i) as
                                            SDL_GameControllerButton) as
                qboolean;
        if pressed as libc::c_uint !=
               stick_state.buttons[i as usize] as libc::c_uint {
            Com_QueueEvent(in_eventTime, SE_KEY, K_PAD0_A as libc::c_int + i,
                           pressed as libc::c_int, 0i32,
                           0 as *mut libc::c_void);
            stick_state.buttons[i as usize] = pressed
        }
        i += 1
    }
    if 0 != (*in_joystickUseAnalog).integer {
        i = 0i32;
        while i < 16i32 {
            translatedAxes[i as usize] = 0i32;
            translatedAxesSet[i as usize] = qfalse;
            i += 1
        }
    }
    i = 0i32;
    while i < SDL_CONTROLLER_AXIS_MAX as libc::c_int {
        let mut axis: libc::c_int =
            SDL_GameControllerGetAxis(gamepad,
                                      (SDL_CONTROLLER_AXIS_LEFTX as
                                           libc::c_int + i) as
                                          SDL_GameControllerAxis) as
                libc::c_int;
        let mut oldAxis: libc::c_int = stick_state.oldaaxes[i as usize];
        let mut f: libc::c_float =
            (abs(axis) as libc::c_float / 32767.0f32 -
                 (*in_joystickThreshold).value) /
                (1.0f32 - (*in_joystickThreshold).value);
        if f < 0.0f32 { f = 0.0f32 }
        axis =
            (32767i32 as libc::c_float * if axis < 0i32 { -f } else { f }) as
                libc::c_int;
        if axis != oldAxis {
            let negMap: [libc::c_int; 6] =
                [K_PAD0_LEFTSTICK_LEFT as libc::c_int,
                 K_PAD0_LEFTSTICK_UP as libc::c_int,
                 K_PAD0_RIGHTSTICK_LEFT as libc::c_int,
                 K_PAD0_RIGHTSTICK_UP as libc::c_int, 0i32, 0i32];
            let posMap: [libc::c_int; 6] =
                [K_PAD0_LEFTSTICK_RIGHT as libc::c_int,
                 K_PAD0_LEFTSTICK_DOWN as libc::c_int,
                 K_PAD0_RIGHTSTICK_RIGHT as libc::c_int,
                 K_PAD0_RIGHTSTICK_DOWN as libc::c_int,
                 K_PAD0_LEFTTRIGGER as libc::c_int,
                 K_PAD0_RIGHTTRIGGER as libc::c_int];
            let mut posAnalog: qboolean = qfalse;
            let mut negAnalog: qboolean = qfalse;
            let mut negKey: libc::c_int = negMap[i as usize];
            let mut posKey: libc::c_int = posMap[i as usize];
            if 0 != (*in_joystickUseAnalog).integer {
                let mut posAxis: libc::c_int = 0i32;
                let mut posSign: libc::c_int = 0i32;
                let mut negAxis: libc::c_int = 0i32;
                let mut negSign: libc::c_int = 0i32;
                posAnalog =
                    KeyToAxisAndSign(posKey, &mut posAxis, &mut posSign);
                negAnalog =
                    KeyToAxisAndSign(negKey, &mut negAxis, &mut negSign);
                if 0 != posAnalog as libc::c_uint &&
                       0 == translatedAxesSet[posAxis as usize] as u64 &&
                       oldAxis > 0i32 && axis <= 0i32 {
                    translatedAxes[posAxis as usize] = 0i32;
                    translatedAxesSet[posAxis as usize] = qtrue
                }
                if 0 != negAnalog as libc::c_uint &&
                       0 == translatedAxesSet[negAxis as usize] as u64 &&
                       oldAxis < 0i32 && axis >= 0i32 {
                    translatedAxes[negAxis as usize] = 0i32;
                    translatedAxesSet[negAxis as usize] = qtrue
                }
                if 0 != posAnalog as libc::c_uint && axis > 0i32 {
                    translatedAxes[posAxis as usize] = axis * posSign;
                    translatedAxesSet[posAxis as usize] = qtrue
                }
                if 0 != negAnalog as libc::c_uint && axis < 0i32 {
                    translatedAxes[negAxis as usize] = -axis * negSign;
                    translatedAxesSet[negAxis as usize] = qtrue
                }
            }
            if 0 == posAnalog as u64 && 0 != posKey && oldAxis > 0i32 &&
                   axis <= 0i32 {
                Com_QueueEvent(in_eventTime, SE_KEY, posKey,
                               qfalse as libc::c_int, 0i32,
                               0 as *mut libc::c_void);
            }
            if 0 == negAnalog as u64 && 0 != negKey && oldAxis < 0i32 &&
                   axis >= 0i32 {
                Com_QueueEvent(in_eventTime, SE_KEY, negKey,
                               qfalse as libc::c_int, 0i32,
                               0 as *mut libc::c_void);
            }
            if 0 == posAnalog as u64 && 0 != posKey && oldAxis <= 0i32 &&
                   axis > 0i32 {
                Com_QueueEvent(in_eventTime, SE_KEY, posKey,
                               qtrue as libc::c_int, 0i32,
                               0 as *mut libc::c_void);
            }
            if 0 == negAnalog as u64 && 0 != negKey && oldAxis >= 0i32 &&
                   axis < 0i32 {
                Com_QueueEvent(in_eventTime, SE_KEY, negKey,
                               qtrue as libc::c_int, 0i32,
                               0 as *mut libc::c_void);
            }
            stick_state.oldaaxes[i as usize] = axis
        }
        i += 1
    }
    if 0 != (*in_joystickUseAnalog).integer {
        i = 0i32;
        while i < 16i32 {
            if 0 != translatedAxesSet[i as usize] as u64 {
                Com_QueueEvent(in_eventTime, SE_JOYSTICK_AXIS, i,
                               translatedAxes[i as usize], 0i32,
                               0 as *mut libc::c_void);
            }
            i += 1
        }
    };
}
unsafe extern "C" fn KeyToAxisAndSign(mut keynum: libc::c_int,
                                      mut outAxis: *mut libc::c_int,
                                      mut outSign: *mut libc::c_int)
 -> qboolean {
    let mut bind: *mut libc::c_char = 0 as *mut libc::c_char;
    if 0 == keynum { return qfalse }
    bind = Key_GetBinding(keynum);
    if bind.is_null() || *bind as libc::c_int != '+' as i32 { return qfalse }
    *outSign = 0i32;
    if Q_stricmp(bind, b"+forward\x00" as *const u8 as *const libc::c_char) ==
           0i32 {
        *outAxis = (*j_forward_axis).integer;
        *outSign = if (*j_forward).value > 0.0f32 { 1i32 } else { -1i32 }
    } else if Q_stricmp(bind,
                        b"+back\x00" as *const u8 as *const libc::c_char) ==
                  0i32 {
        *outAxis = (*j_forward_axis).integer;
        *outSign = if (*j_forward).value > 0.0f32 { -1i32 } else { 1i32 }
    } else if Q_stricmp(bind,
                        b"+moveleft\x00" as *const u8 as *const libc::c_char)
                  == 0i32 {
        *outAxis = (*j_side_axis).integer;
        *outSign = if (*j_side).value > 0.0f32 { -1i32 } else { 1i32 }
    } else if Q_stricmp(bind,
                        b"+moveright\x00" as *const u8 as *const libc::c_char)
                  == 0i32 {
        *outAxis = (*j_side_axis).integer;
        *outSign = if (*j_side).value > 0.0f32 { 1i32 } else { -1i32 }
    } else if Q_stricmp(bind,
                        b"+lookup\x00" as *const u8 as *const libc::c_char) ==
                  0i32 {
        *outAxis = (*j_pitch_axis).integer;
        *outSign = if (*j_pitch).value > 0.0f32 { -1i32 } else { 1i32 }
    } else if Q_stricmp(bind,
                        b"+lookdown\x00" as *const u8 as *const libc::c_char)
                  == 0i32 {
        *outAxis = (*j_pitch_axis).integer;
        *outSign = if (*j_pitch).value > 0.0f32 { 1i32 } else { -1i32 }
    } else if Q_stricmp(bind,
                        b"+left\x00" as *const u8 as *const libc::c_char) ==
                  0i32 {
        *outAxis = (*j_yaw_axis).integer;
        *outSign = if (*j_yaw).value > 0.0f32 { 1i32 } else { -1i32 }
    } else if Q_stricmp(bind,
                        b"+right\x00" as *const u8 as *const libc::c_char) ==
                  0i32 {
        *outAxis = (*j_yaw_axis).integer;
        *outSign = if (*j_yaw).value > 0.0f32 { -1i32 } else { 1i32 }
    } else if Q_stricmp(bind,
                        b"+moveup\x00" as *const u8 as *const libc::c_char) ==
                  0i32 {
        *outAxis = (*j_up_axis).integer;
        *outSign = if (*j_up).value > 0.0f32 { 1i32 } else { -1i32 }
    } else if Q_stricmp(bind,
                        b"+movedown\x00" as *const u8 as *const libc::c_char)
                  == 0i32 {
        *outAxis = (*j_up_axis).integer;
        *outSign = if (*j_up).value > 0.0f32 { -1i32 } else { 1i32 }
    }
    return (*outSign != 0i32) as libc::c_int as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn IN_Shutdown() {
    SDL_StopTextInput();
    IN_DeactivateMouse((Cvar_VariableIntegerValue(b"r_fullscreen\x00" as
                                                      *const u8 as
                                                      *const libc::c_char) !=
                            0i32) as libc::c_int as qboolean);
    mouseAvailable = qfalse;
    IN_ShutdownJoystick();
    SDL_window = 0 as *mut SDL_Window;
}
/*
===============
IN_ShutdownJoystick
===============
*/
unsafe extern "C" fn IN_ShutdownJoystick() {
    if 0 == SDL_WasInit(0x2000u32) { return }
    if 0 == SDL_WasInit(0x200u32) { return }
    if !gamepad.is_null() {
        SDL_GameControllerClose(gamepad);
        gamepad = 0 as *mut SDL_GameController
    }
    if !stick.is_null() {
        SDL_JoystickClose(stick);
        stick = 0 as *mut SDL_Joystick
    }
    SDL_QuitSubSystem(0x2000u32);
    SDL_QuitSubSystem(0x200u32);
}
#[no_mangle]
pub unsafe extern "C" fn IN_Restart() {
    IN_ShutdownJoystick();
    IN_Init(SDL_window as *mut libc::c_void);
}