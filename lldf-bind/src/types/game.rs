use crate::prelude::*;
use crate::bind::DFOpaqueValue;


pub struct Game {
    _unconstructable : ()
}

impl Game {

    // TODO: Move to `PlayerSel` as constructor.
    #[inline(always)]
    pub fn all_players() -> PlayerSel { unsafe {
        let uuids = DF_GAMEVALUE__PlotPlayerUUIDs_Default();
        PlayerSel { uuids }
    } }

    #[inline(always)]
    pub fn player_by_uuid<T : AsRef<String>>(uuid : T) -> PlayerSel { unsafe {
        DF_ACTION__SelectObject_PlayerName(uuid.as_ref() as *const _ as *const _);
        let uuids = DF_GAMEVALUE__SelectionTargetUUIDs_Default();
        DF_ACTION__SelectObject_Reset();
        PlayerSel { uuids }
    } }

}

/// `SET_VARIABLE` / `Variable Setting`
impl Game {

    // TODO: SetPersistentData

    // TODO: GetPersistentData

    // TODO: RemovePersistentData

}

/// `START_PROCESS`
impl Game {

    // TODO: StartProcess
    // Probably a method which takes a function, then calls the function through a process.

}

/// `CONTROL`
impl Game {

    // TODO: Wait

    // TODO: End

}





#[allow(clashing_extern_declarations)]
extern "C" {

    fn DF_GAMEVALUE__PlotPlayerUUIDs_Default( ) -> List<String>;
    fn DF_GAMEVALUE__SelectionTargetUUIDs_Default( ) -> List<String>;

    fn DF_ACTION__SelectObject_PlayerName( target : *const DFOpaqueValue ) -> ();

    fn DF_ACTION__SelectObject_Reset( ) -> ();

}
