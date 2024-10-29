use crate::prelude::*;


pub enum Game { /* Unconstructable */ }

#[allow(deprecated)]
impl Game {

    #[inline(always)]
    pub fn all_players() -> PlayerSel { unsafe {
        DF_ACTION__SelectObject_AllPlayers();
        let uuids = crate::bind::gamevalue::DF_GAMEVALUE__SelectionTargetUUIDs_Default() as *const List<String>;
        DF_ACTION__SelectObject_Reset();
        PlayerSel { uuids }
    } }

    #[inline(always)]
    pub fn default_player() -> PlayerSel { unsafe {
        DF_ACTION__SelectObject_EventTarget_EventTarget_Default();
        let uuids = crate::bind::gamevalue::DF_GAMEVALUE__SelectionTargetUUIDs_Default() as *const List<String>;
        DF_ACTION__SelectObject_Reset();
        PlayerSel { uuids }
    } }

}

#[allow(deprecated)]
impl Game {

    #[inline(always)]
    pub fn player_count() -> UInt { unsafe {
        *(crate::bind::gamevalue::DF_GAMEVALUE__PlayerCount_Default() as *const UInt)
    } }

}





extern "C" {

    fn DF_ACTION__SelectObject_EventTarget_EventTarget_Default( ) -> ();

    fn DF_ACTION__SelectObject_AllPlayers( ) -> ();

    fn DF_ACTION__SelectObject_Reset( ) -> ();

}
