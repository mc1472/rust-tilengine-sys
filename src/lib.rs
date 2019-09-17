#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[repr(C)]
pub struct TLN_SpriteState {
    x : isize,
    y : isize, 
    w : isize,
    h : isize,
    flags : TLN_TileFlags,
    palatte : TLN_Palette,
    spriteset : TLN_Spriteset,
    index : isize,
    enabled : bool,
    collision : bool
}

#[repr(C)]
pub enum TLN_LogLevel {
    TLN_LOG_NONE,
    TLN_LOG_ERRORS,
    TLN_LOG_VERBOSE,
}

extern {
    pub fn TLN_GetSpriteStates(nsprite: std::os::raw::c_int, state : *mut TLN_SpriteState) -> bool_;
    pub fn TLN_SetLogLevel(log_level : TLN_LogLevel);
}





