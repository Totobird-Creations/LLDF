use crate::prelude::*;


#[repr(transparent)]
pub struct PlayerSel {
    pub(in super::super) uuids : *const List<String>
}

#[allow(deprecated)]
impl PlayerSel {

    #[inline(always)]
    pub fn uuids<'l>(&'l self) -> &'l List<String> { unsafe{ &*self.uuids } }

    #[inline(always)]
    pub fn names(&self) -> &List<Text> { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids);
        let names = crate::bind::gamevalue::DF_GAMEVALUE__SelectionTargetNames_Default() as *const List<Text>;
        DF_ACTION__SelectObject_Reset();
        &*names
    } }

}

#[allow(deprecated)]
impl PlayerSel {

    #[inline(always)]
    pub fn send_message<T : AsRef<Text>>(&self, text : T) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids);
        DF_ACTION__PlayerAction_SendMessage_AlignmentMode_Regular_TextValueMerging_NoSpaces_InheritStyles_False(text.as_ref());
        DF_ACTION__SelectObject_Reset();
    } }

}


unsafe impl DFSel for PlayerSel {}





extern "C" {

    fn DF_ACTION__SelectObject_PlayerName( uuids : *const List<String> ) -> ();

    fn DF_ACTION__SelectObject_Reset( ) -> ();

    fn DF_ACTION__PlayerAction_SendMessage_AlignmentMode_Regular_TextValueMerging_NoSpaces_InheritStyles_False( message : *const Text ) -> ();

}
