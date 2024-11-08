use crate::prelude::*;
use crate::bind::DFOpaqueValue;


pub enum Game { /* Unconstructable */ }

impl Game {

    #[inline(always)]
    pub fn all_players() -> PlayerSel { unsafe {
        let uuids = crate::bind::gamevalue::DF_GAMEVALUE__PlotPlayerUUIDs_Default();
        PlayerSel { uuids : DF_TRANSMUTE__ListString(uuids) }
    } }

    #[inline(always)]
    pub fn player_by_uuid<T : AsRef<String>>(uuid : T) -> PlayerSel { unsafe {
        DF_ACTION__SelectObject_PlayerName(uuid.as_ref() as *const _ as *const _);
        let uuids = crate::bind::gamevalue::DF_GAMEVALUE__SelectionTargetUUIDs_Default();
        DF_ACTION__SelectObject_Reset();
        PlayerSel { uuids : DF_TRANSMUTE__ListString(uuids) }
    } }

}

// `SET_VARIABLE` / `Variable Setting`
impl Game {

    // TODO: SetPersistentData

    // TODO: GetPersistentData

    // TODO: RemovePersistentData

}

impl Game {

    #[inline(always)]
    pub fn player_count() -> UInt { unsafe {
        DF_TRANSMUTE__UInt(crate::bind::gamevalue::DF_GAMEVALUE__PlayerCount_Default())
    } }

}





#[allow(clashing_extern_declarations)]
extern "C" {

    fn DF_TRANSMUTE__UInt( from : DFOpaqueValue ) -> UInt;
    fn DF_TRANSMUTE__ListString( from : DFOpaqueValue ) -> List<String>;

    fn DF_ACTION__SelectObject_PlayerName( target : *const DFOpaqueValue ) -> ();

    fn DF_ACTION__SelectObject_Reset( ) -> ();

}
