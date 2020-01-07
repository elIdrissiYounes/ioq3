#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_match_s {
    pub string: [i8; 256],
    pub type_0: i32,
    pub subtype: i32,
    pub variables: [crate::be_ai_chat_h::bot_matchvariable_t; 8],
}
// true if updated this frame

// entity type

// entity flags

// local time

// time between last and current update

// number of the entity

// origin of the entity

// angles of the model

// for lerping

// last visible origin

// bounding box minimums

// bounding box maximums

// ground entity

// solid type

// model used

// weapons, CTF flags, etc

// model frame number

// impulse events -- muzzle flashes, footsteps, etc

// even parameter

// bit flags

// determines weapon and flash model, etc

// mask off ANIM_TOGGLEBIT

// mask off ANIM_TOGGLEBIT

//match variable
pub type bot_matchvariable_t = crate::be_ai_chat_h::bot_matchvariable_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_matchvariable_s {
    pub offset: i8,
    pub length: i32,
}
//returned to AI when a match is found
pub type bot_match_t = crate::be_ai_chat_h::bot_match_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_consolemessage_s {
    pub handle: i32,
    pub time: f32,
    pub type_0: i32,
    pub message: [i8; 256],
    pub prev: *mut crate::be_ai_chat_h::bot_consolemessage_s,
    pub next: *mut crate::be_ai_chat_h::bot_consolemessage_s,
}
//a console message
pub type bot_consolemessage_t = crate::be_ai_chat_h::bot_consolemessage_s;
