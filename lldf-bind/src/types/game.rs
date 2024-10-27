use crate::prelude::*;
use crate::core::mem::transmute_unchecked;


pub enum Game { /* Unconstructable */ }

#[allow(deprecated)]
impl Game {

    #[inline(always)]
    pub fn all_players() -> PlayerSel { unsafe {
        crate::bind::action::DF_ACTION_SelectObject_AllPlayers();
        let uuids = *transmute_unchecked::<_, &_>(&crate::bind::gamevalue::DF_GAMEVALUE_SelectionTargetUUIDs_Default());
        crate::bind::action::DF_ACTION_SelectObject_Reset();
        PlayerSel { uuids }
    } }

    #[inline(always)]
    pub fn default_player() -> PlayerSel { unsafe {
        //crate::bind::action::DF_ACTION_SelectObject_DefaultPlayer();
        crate::bind::action::DF_ACTION_SelectObject_EventTarget_EventTarget("Default".into());
        let uuids = *transmute_unchecked::<_, &_>(&crate::bind::gamevalue::DF_GAMEVALUE_SelectionTargetUUIDs_Default());
        crate::bind::action::DF_ACTION_SelectObject_Reset();
        PlayerSel { uuids }
    } }

}

#[allow(deprecated)]
impl Game {

    #[inline(always)]
    pub fn player_count() -> UInt { unsafe {
        *transmute_unchecked::<_, &_>(&crate::bind::gamevalue::DF_GAMEVALUE_PlayerCount_Default())
    } }

    #[inline(always)]
    pub fn sleep<U : Into<UInt>>(amount : U) -> () { unsafe {
        crate::bind::action::DF_ACTION_Control_Wait_TimeUnit("Ticks".into(), amount);
    } }

}
