#![allow(clashing_extern_declarations)]

//! *Automatically generated using the action dump.*

//pub mod sound;
//pub mod particle;
mod _action;
pub mod action {
    pub use super::_action::*;
    extern "C" {

        pub fn DF_ACTION__SelectObject_EventTarget_EventTarget_Default( ) -> ();

    }
}
pub mod gamevalue;


/// A value returned by a DiamondFire operation.
pub struct DFOpaqueValue {
    _opaque_type : u8
}
