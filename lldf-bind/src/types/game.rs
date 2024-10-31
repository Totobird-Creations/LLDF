use crate::prelude::*;
use crate::bind::DFOpaqueValue;


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
    pub fn player_by_uuid<T : AsRef<String>>(uuid : T) -> PlayerSel { unsafe {
        DF_ACTION__SelectObject_PlayerName(uuid.as_ref() as *const _ as *const _);
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





#[allow(clashing_extern_declarations)]
extern "C" {

    fn DF_ACTION__SelectObject_PlayerName( target : *const DFOpaqueValue ) -> ();

    fn DF_ACTION__SelectObject_AllPlayers( ) -> ();

    fn DF_ACTION__SelectObject_Reset( ) -> ();

}
