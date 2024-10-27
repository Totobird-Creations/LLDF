use crate::prelude::*;
use crate::core::mem::transmute_unchecked;


#[repr(transparent)]
pub struct PlayerSel {
    pub(in super::super) uuids : List<String>
}

#[allow(deprecated)]
impl PlayerSel {

    #[inline(always)]
    pub fn uuids(&self) -> List<String> { self.uuids }

    #[inline(always)]
    pub fn names(&self) -> List<Text> { unsafe {
        crate::bind::action::DF_ACTION_SelectObject_PlayerName(self.uuids);
        let names = *transmute_unchecked::<_, &_>(&crate::bind::gamevalue::DF_GAMEVALUE_SelectionTargetNames_Default());
        crate::bind::action::DF_ACTION_SelectObject_Reset();
        names
    } }

}

#[allow(deprecated)]
impl PlayerSel {

    #[inline(always)]
    pub fn send_message<T : Into<Text>>(&self, text : T) -> () { unsafe {
        crate::bind::action::DF_ACTION_SelectObject_PlayerName(self.uuids);
        crate::bind::action::DF_ACTION_PlayerAction_SendMessage_AlignmentMode_TextValueMerging_InheritStyles("Regular".into(), "No spaces".into(), "False".into(), text.into());
        crate::bind::action::DF_ACTION_SelectObject_Reset();
    } }

    #[inline(always)]
    pub fn play_sound(&self, sound : Sound) -> () { unsafe {
        crate::bind::action::DF_ACTION_SelectObject_PlayerName(self.uuids);
        crate::bind::action::DF_ACTION_PlayerAction_PlaySound_SoundSource("Master".into(), sound); // TODO: Add some way for the user to change the sound source.
        crate::bind::action::DF_ACTION_SelectObject_Reset();
    } }

    #[inline(always)]
    pub fn kick(&self) -> () { unsafe {
        crate::bind::action::DF_ACTION_SelectObject_PlayerName(self.uuids);
        crate::bind::action::DF_ACTION_PlayerAction_Kick();
        crate::bind::action::DF_ACTION_SelectObject_Reset();
    } }

}


unsafe impl DFSel for PlayerSel {}
