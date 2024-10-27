//! *Automatically generated using the action dump.*

pub mod sound;
pub mod particle;
pub mod action;
pub mod gamevalue;

pub mod bracket { extern "C" {
    pub fn DF_BRACKET_Normal_Open() -> ();
    pub fn DF_BRACKET_Normal_Close() -> ();
    pub fn DF_BRACKET_Repeat_Open() -> ();
    pub fn DF_BRACKET_Repeat_Close() -> ();
} }


/// A value returned by a DiamondFire operation.
pub struct DFOpaqueValue {
    _opaque_type : u8
}
