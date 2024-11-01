#![allow(unused)]


use crate::prelude::*;
pub mod bind { pub use crate::bind::*; }


#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum Calling_an_event_trigger_is_not_allowed {}


pub macro event_trigger {


    { Join, $original_func_ident:ident } => { event_trigger_inner!{ DF_EVENT__Event_Join, $original_func_ident, {
        {
            unsafe{ ::lldf_bind::__private::bind::action::DF_ACTION__SelectObject_EventTarget_EventTarget_Default(); }
            let uuids = unsafe{ ::lldf_bind::__private::bind::gamevalue::DF_GAMEVALUE__SelectionTargetUUIDs_Default() } as *const List<String>;
            unsafe{ ::lldf_bind::__private::bind::action::DF_ACTION__SelectObject_Reset(); }
            ::lldf_bind::types::sel::player::PlayerSel { uuids }
        }
    }} },

    { Leave, $original_func_ident:ident } => { event_trigger_inner!{ DF_EVENT__Event_Leave, $original_func_ident, {
        {
            unsafe{ ::lldf_bind::__private::bind::action::DF_ACTION__SelectObject_EventTarget_EventTarget_Default(); }
            let uuids = unsafe{ ::lldf_bind::__private::bind::gamevalue::DF_GAMEVALUE__SelectionTargetUUIDs_Default() } as *const List<String>;
            unsafe{ ::lldf_bind::__private::bind::action::DF_ACTION__SelectObject_Reset(); }
            ::lldf_bind::types::sel::player::PlayerSel { uuids }
        }
    }} },

    { Command, $original_func_ident:ident } => { event_trigger_inner!{ DF_EVENT__Event_Command, $original_func_ident, {
        {
            unsafe{ ::lldf_bind::__private::bind::action::DF_ACTION__SelectObject_EventTarget_EventTarget_Default(); }
            let uuids = unsafe{ ::lldf_bind::__private::bind::gamevalue::DF_GAMEVALUE__SelectionTargetUUIDs_Default() } as *const List<String>;
            unsafe{ ::lldf_bind::__private::bind::action::DF_ACTION__SelectObject_Reset(); }
            ::lldf_bind::types::sel::player::PlayerSel { uuids }
        }
    }} },


    { SwapHands, $original_func_ident:ident } => { event_trigger_inner!{ DF_EVENT__Event_SwapHands, $original_func_ident, {
        {
            unsafe{ ::lldf_bind::__private::bind::action::DF_ACTION__SelectObject_EventTarget_EventTarget_Default(); }
            let uuids = unsafe{ ::lldf_bind::__private::bind::gamevalue::DF_GAMEVALUE__SelectionTargetUUIDs_Default() } as *const List<String>;
            unsafe{ ::lldf_bind::__private::bind::action::DF_ACTION__SelectObject_Reset(); }
            ::lldf_bind::types::sel::player::PlayerSel { uuids }
        }
    }} },


    { $event:ident, $original_func_ident:ident } => {
        compile_error!(concat!("Unknown event `", stringify!($event), "`"));
        event_trigger_inner!{ $trigger_func_ident, $original_func_ident }
    }
}


macro event_trigger_inner {
    ( $trigger_func_ident:ident, $original_func_ident:ident, { $( $arg:expr ),* } ) => {

        #[no_mangle]
        extern "C" fn $trigger_func_ident(_ : ::lldf_bind::__private::Calling_an_event_trigger_is_not_allowed) {
            $original_func_ident(
                unsafe{ *(::lldf_bind::core::mem::transmute_unchecked::<_, &_>(&())) }
                $( , $arg )*
            );
        }

    }
}
